use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use webarkitlib_rs::ar2::{ar2_gen_feature_map, ar2_gen_image_set};

#[cfg(feature = "ffi-backend")]
const KPM_SURF_FEATURE_DENSITY: i32 = 100;

/// Generate a WebARKit-compatible NFT marker payload from image bytes.
///
/// This function coordinates the generation of image sets (`.iset`) and feature sets (`.fset`, `.fset3`) 
/// which are required for NFT tracking in WebARKit.
///
/// # Arguments
///
/// * `image_data` - Raw RGB byte slice of the source image.
/// * `image_width` - Width of the image in pixels.
/// * `image_height` - Height of the image in pixels.
/// * `image_nc` - Number of channels (usually 3 for RGB).
/// * `output_dir` - Directory where the generated marker files will be saved.
/// * `marker_name` - Base name of the marker. The generated files will use this name.
/// * `dpi` - DPI (Dots Per Inch) value of the source image.
/// * `progress` - Optional `AtomicU32` wrapped in an `Arc` to report progress updates (0-100%).
///
/// # Errors
/// 
/// Returns an error if generating the image set, feature map, or FREAK features fail.
pub fn generate_nft_marker(
    image_data: &[u8],
    image_width: i32,
    image_height: i32,
    image_nc: i32,
    output_dir: &std::path::Path,
    marker_name: &str,
    dpi: f32,
    progress: Option<Arc<AtomicU32>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let set_progress = |val: u32| {
        if let Some(p) = &progress {
            p.store(val, Ordering::SeqCst);
        }
    };

    set_progress(5); // Started
    let iset_path = output_dir.join(format!("{}.iset", marker_name));
    let fset_path = output_dir.join(format!("{}.fset", marker_name));
    let fset3_path = output_dir.join(format!("{}.fset3", marker_name));

    set_progress(10);
    let image_set = ar2_gen_image_set(image_data, image_width, image_height, image_nc, dpi)?;

    set_progress(20);
    // AR2ImageSetT::save writes to a Write trait object.
    let ret = image_set.save(iset_path.to_str().ok_or("Invalid output path")?)?;

    set_progress(30);
    let feature_set = ar2_gen_feature_map(&image_set, 10, 250, 2).map_err(|e| {
        format!("ar2_gen_feature_map failed: {}", e)
    })?;

    set_progress(40);
    feature_set
        .save(fset_path.to_str().ok_or("Invalid output path")?)
        .map_err(|e| format!("failed to save {:?}: {}", fset_path, e))?;
    #[cfg(feature = "ffi-backend")]
    {
        use webarkitlib_rs::kpm::types::KpmRefDataSet;
        use webarkitlib_rs::kpm::{CppFreakMatcher, KpmCompMode, KpmError, KpmProcMode};

        println!("Generating FeatureSet3...");

        let mut combined: Option<KpmRefDataSet> = None;

        let total_scales = image_set.scale.len() as f32;
        for (image_no, scale) in image_set.scale.iter().enumerate() {
            // Progress from 40% to 90%
            let current_progress = 40 + ((image_no as f32 / total_scales) * 50.0) as u32;
            set_progress(current_progress);
            
            // Max features proportional to image area (matching markerCreator.cpp).
            let max_feat = KPM_SURF_FEATURE_DENSITY * scale.xsize * scale.ysize / (480 * 360);

            // Fresh matcher for each scale (scales may have different dimensions).
            let mut matcher = CppFreakMatcher::new(scale.xsize, scale.ysize).map_err(|e| {
                format!("CppFreakMatcher::new failed: {:?}", e)
            })?;

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

        println!("Saving FeatureSet3 to {:?}...", fset3_path);
        if let Some(ref_data) = combined {
            let total_kpm = ref_data.num as usize;
            ref_data.save(&fset3_path).map_err(|e| {
                format!("failed to save {:?}: {}", fset3_path, e)
            })?;
            //let save_elapsed = save_start.elapsed().as_secs_f64();
            println!(
                "  Done. {} FREAK features total  ({})",
                total_kpm,
                120,
            );
        } else {
            eprintln!("Warning: no KPM features extracted — .fset3 not written.");
        }
    }
    set_progress(100);
    Ok(ret)
}
