use webarkitlib_rs::ar2::{ar2_gen_feature_map, ar2_gen_image_set};

#[cfg(feature = "ffi-backend")]
const KPM_SURF_FEATURE_DENSITY: i32 = 100;

/// Generate a WebARKit-compatible NFT marker payload from image bytes.
pub fn generate_nft_marker(
    image_data: &[u8],
    image_width: i32,
    image_height: i32,
    image_nc: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_set = ar2_gen_image_set(image_data, image_width, image_height, image_nc, 220.0)?;

    // AR2ImageSetT::save writes to a Write trait object. We can use a
    let ret = image_set.save("pinball.iset")?;

    let feature_set = ar2_gen_feature_map(&image_set, 10, 250, 2).unwrap_or_else(|e| {
        eprintln!("Error: ar2_gen_feature_map failed: {}", e);
        std::process::exit(1);
    });
    println!("Saving FeatureSet to {:?}...", "there");
    //let save_start = Instant::now();
    feature_set.save("pinball.fset").unwrap_or_else(|e| {
        eprintln!("Error: failed to save {:?}: {}", "there", e);
        std::process::exit(1);
    });
    #[cfg(feature = "ffi-backend")]
    {
        use webarkitlib_rs::kpm::types::KpmRefDataSet;
        use webarkitlib_rs::kpm::{CppFreakMatcher, KpmCompMode, KpmError, KpmProcMode};

        println!("Generating FeatureSet3...");

        let mut combined: Option<KpmRefDataSet> = None;

        for (image_no, scale) in image_set.scale.iter().enumerate() {
            // Max features proportional to image area (matching markerCreator.cpp).
            let max_feat = KPM_SURF_FEATURE_DENSITY * scale.xsize * scale.ysize / (480 * 360);

            // Fresh matcher for each scale (scales may have different dimensions).
            let mut matcher = CppFreakMatcher::new(scale.xsize, scale.ysize).unwrap_or_else(|e| {
                eprintln!("Error: CppFreakMatcher::new failed: {:?}", e);
                std::process::exit(1);
            });

            let result = KpmRefDataSet::generate(
                &scale.img_bw,
                scale.xsize,
                scale.ysize,
                scale.dpi,
                KpmProcMode::FullSize,
                KpmCompMode::None,
                max_feat,
                1, // page_no = 1 (matching markerCreator.cpp)
                image_no as i32,
                &mut matcher,
            );

            match result {
                Ok(ref_data) => {
                    println!(
                        "  ({}, {}) {:.6} dpi: Freak features - {}",
                        scale.xsize, scale.ysize, scale.dpi, ref_data.num
                    );
                    combined = Some(match combined.take() {
                        Some(mut existing) => {
                            existing.merge(ref_data);
                            existing
                        }
                        None => ref_data,
                    });
                }
                Err(KpmError::InvalidInput(_)) => {
                    println!(
                        "  ({}, {}) {:.6} dpi: Freak features - 0 (scale too small)",
                        scale.xsize, scale.ysize, scale.dpi
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Warning: KpmRefDataSet::generate failed for scale {}: {:?}",
                        image_no, e
                    );
                }
            }
        }

        println!();

        //let gen_elapsed = step_start.elapsed().as_secs_f64();

        //println!("Saving FeatureSet3 to {:?}...", fset3_path);
        println!("Saving FeatureSet3 to {:?}...", "there");
        if let Some(ref_data) = combined {
            let total_kpm = ref_data.num as usize;
            //let save_start = Instant::now();
            ref_data.save(std::path::Path::new("pinball.fset3")).unwrap_or_else(|e| {
                eprintln!("Error: failed to save {:?}: {}", "pinball.fset3", e);
                std::process::exit(1);
            });
            //let save_elapsed = save_start.elapsed().as_secs_f64();
            println!(
                "  Done. {} FREAK features total  ({})",
                total_kpm,
                120,
                //file_kb(&fset3_path),
            );
            /*println!(
                "  FREAK extraction: {:.1}s | Save: {:.1}s | Total: {:.1}s",
                gen_elapsed,
                save_elapsed,
                step_start.elapsed().as_secs_f64()
            );*/
        } else {
            eprintln!("Warning: no KPM features extracted — .fset3 not written.");
        }
    }
    Ok(ret)
}
