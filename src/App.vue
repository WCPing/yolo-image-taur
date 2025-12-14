<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 定义响应式引用
const selectedFile = ref<File | null>(null);
const imageInput = ref<HTMLInputElement>();
const processBtn = ref<HTMLButtonElement>();
const originalImage = ref<HTMLImageElement>();
const resultImage = ref<HTMLImageElement>();
const statusText = ref<HTMLParagraphElement>();

// 初始化状态
const isProcessing = ref(false);
const originalImageSrc = ref<string>('');
const resultImageSrc = ref<string>('');
const statusMessage = ref('请选择图片并点击处理');
const statusClass = ref('');
const showStatus = ref(true);
const showDownloadButton = ref(false);
const showDownloadSuccess = ref(false);

const onImageChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (file) {
        selectedFile.value = file;
        const reader = new FileReader();
        reader.onload = (e) => {
            originalImageSrc.value = e.target?.result as string;
            statusMessage.value = `已选择文件: ${file.name}`;
            statusClass.value = '';
            isProcessing.value = false;
            showStatus.value = true;
            showDownloadButton.value = false;
        };
        reader.readAsDataURL(file);
    }
}

const reset = () => {
    selectedFile.value = null;
    if (imageInput.value) {
        imageInput.value.value = '';
    }
    originalImageSrc.value = '';
    resultImageSrc.value = '';
    statusMessage.value = '请选择图片并点击处理';
    statusClass.value = '';
    isProcessing.value = false;
    showStatus.value = true;
    showDownloadButton.value = false;
    showDownloadSuccess.value = false;
}

const downloadImage = () => {
    if (resultImageSrc.value) {
        const link = document.createElement('a');
        link.href = resultImageSrc.value;
        link.download = 'yolo_detection_result.png';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);

        // 显示下载成功提示
        showDownloadSuccess.value = true;
        setTimeout(() => {
            showDownloadSuccess.value = false;
        }, 2000);
    }
}

const uploadImage = async () => {
    if (!selectedFile.value) {
        statusMessage.value = '请先选择图片文件';
        statusClass.value = 'error';
        return;
    }

    try {
        // 显示加载状态
        isProcessing.value = true;
        statusMessage.value = '正在处理图片...';
        statusClass.value = 'loading';

        // 读取文件为字节数组
        const arrayBuffer = await selectedFile.value.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        // 调用Rust后端的YOLO处理函数
        const resultBytes = await invoke('process_image', {
            inputData: Array.from(uint8Array)
        }) as number[];

        // 将结果字节转换为图片URL
        const resultBlob = new Blob([new Uint8Array(resultBytes)], { type: 'image/png' });
        resultImageSrc.value = URL.createObjectURL(resultBlob);

        statusMessage.value = '处理完成！';
        statusClass.value = 'success';
        showStatus.value = false;
        showDownloadButton.value = true;

    } catch (error) {
        console.error('处理失败:', error);
        statusMessage.value = `处理失败: ${error}`;
        statusClass.value = 'error';
    } finally {
        isProcessing.value = false;
    }
}

</script>

<template>
    <div class="container">
        <h1>YOLO 目标检测</h1>

        <div class="upload-section">
            <input type="file" ref="imageInput" accept="image/*" @change="onImageChange">
            <button ref="processBtn" :disabled="isProcessing" @click="uploadImage">处理图片</button>
            <button class="reset-btn" @click="reset" :disabled="isProcessing">重置</button>
        </div>

        <div class="result-section">
            <div class="image-container">
                <h3>原始图片</h3>
                <img ref="originalImage" :src="originalImageSrc" :style="{ display: originalImageSrc ? 'block' : 'none' }" alt="原始图片">
            </div>

            <div class="image-container">
                <h3>检测结果</h3>
                <img ref="resultImage" :src="resultImageSrc" :style="{ display: resultImageSrc ? 'block' : 'none' }" alt="检测结果">
            </div>
        </div>

        <div class="status">
            <p ref="statusText" v-show="showStatus" :class="statusClass">{{ statusMessage }}</p>
            <button v-show="showDownloadButton" @click="downloadImage">下载图片</button>
            <p v-show="showDownloadSuccess" class="download-success">下载成功！</p>
        </div>
    </div>
</template>

<style scoped>
.container {
    max-width: 1200px;
    margin: 0 auto;
    background: white;
    padding: 30px;
    border-radius: 10px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

h1 {
    text-align: center;
    color: #333;
    margin-bottom: 30px;
}

.upload-section {
    text-align: center;
    margin-bottom: 30px;
}

.upload-section input[type="file"] {
    margin-right: 15px;
    padding: 8px;
    border: 2px dashed #ddd;
    border-radius: 5px;
    background: #fafafa;
}

.container button {
    background-color: #007bff;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
    transition: background-color 0.3s;
}

.container button:hover {
    background-color: #0056b3;
}

.container button:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
}

.upload-section .reset-btn {
    background-color: white;
    color: #333;
    border: 1px solid #ddd;
}

.upload-section .reset-btn:hover {
    background-color: #f8f9fa;
}

.upload-section .reset-btn:disabled {
    background-color: #f8f9fa;
    color: #6c757d;
    border-color: #ddd;
    cursor: not-allowed;
}

.result-section {
    display: flex;
    justify-content: space-around;
    margin-bottom: 30px;
    flex-wrap: wrap;
}

.image-container {
    flex: 1;
    min-width: 300px;
    margin: 10px;
    text-align: center;
}

.image-container h3 {
    color: #555;
    margin-bottom: 15px;
}

.image-container img {
    max-width: 100%;
    max-height: 400px;
    border: 2px solid #ddd;
    border-radius: 5px;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

.status {
    text-align: center;
    padding: 15px;
    background-color: #e9ecef;
    border-radius: 5px;
    margin-top: 20px;
}

.status p {
    margin: 0;
    color: #495057;
    font-weight: 500;
}

.loading {
    color: #007bff;
}

.success {
    color: #28a745;
}

.error {
    color: #dc3545;
}

.download-success {
    color: #28a745;
    font-weight: 500;
    animation: fadeIn 0.3s ease-in;
}

@keyframes fadeIn {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}
</style>

<style>
body {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    margin: 0;
    padding: 20px;
    background-color: #f5f5f5;
}
</style>
