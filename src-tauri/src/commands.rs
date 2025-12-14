use image::{DynamicImage, ImageFormat};
use yolov10::{InferenceEngine, draw_labels};
use std::io::Cursor;

// 处理图片并进行YOLO目标检测的函数
#[tauri::command]
pub async fn process_image(input_data: Vec<u8>) -> Result<Vec<u8>, String> {
    println!("Processing image: {} bytes", input_data.len());

    // 初始化YOLO推理引擎
    let mut engine = InferenceEngine::new("yolov10s.onnx")
        .map_err(|e| format!("Failed to create inference engine: {}", e))?;

    // 执行目标检测推理
    let results = engine.run_inference(&input_data, 0.3)
        .map_err(|e| format!("Failed to run inference: {}", e))?;

    // 加载图片用于可视化
    let image = image::load_from_memory(&input_data)
        .map_err(|e| format!("Failed to load image: {}", e))?;

    // 在图片上绘制检测结果
    let img: DynamicImage = draw_labels(&image, &results);

    // 保存结果图片
    let mut output = Vec::new();
    img.write_to(&mut Cursor::new(&mut output), ImageFormat::Png)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    println!("Successfully processed image, output size: {} bytes", output.len());
    Ok(output)
}
