import os
from ultralytics import YOLO

def main():
    """
    Main function to run the YOLO model fine-tuning process.
    """
    # --- 1. Configuration ---

    # Path to the pretrained model file.
    # The user mentioned a YOLOv11 .pt file. The ultralytics library is flexible
    # and can often load weights from various YOLO versions.
    # Replace 'yolov11.pt' with the actual path to your model file.
    # If you don't have it yet, you can start with an official model like 'yolov8n.pt'.
    pretrained_model_path = 'yolo11m.pt' # <-- IMPORTANT: Change this to your yolov11.pt file

    # Path to your dataset configuration YAML file.
    dataset_config_path = os.path.join('dataset', 'data.yaml')

    # Training hyperparameters.
    # Adjust these based on your specific needs and hardware.
    epochs = 100
    batch_size = 16
    image_size = 640

    # --- 2. Load the Model ---

    # Load a pretrained YOLO model.
    # The '.pt' file contains the model architecture and the pretrained weights.
    print(f"Loading pretrained model from: {pretrained_model_path}")
    try:
        model = YOLO(pretrained_model_path)
    except Exception as e:
        print(f"Error loading model: {e}")
        print("Please ensure the model path is correct and the file is not corrupted.")
        return

    # --- 3. Train the Model ---

    print("Starting model fine-tuning...")
    print(f"Dataset: {dataset_config_path}")
    print(f"Epochs: {epochs}, Batch Size: {batch_size}, Image Size: {image_size}")

    # Start the training process.
    # The results will be saved to a 'runs/detect/train' directory by default.
    model.train(
        data=dataset_config_path,
        epochs=epochs,
        batch=batch_size,
        imgsz=image_size,
        # The user mentioned contrast-adjusted images. Ultralytics performs
        # its own augmentations by default. If your dataset is already heavily
        # augmented, you might want to reduce or disable some of these.
        # For example, to disable mosaic augmentation:
        # mosaic=0.0,
        # To see all options, check the ultralytics documentation.
        project='runs/detect',  # Directory to save results
        name='yolo_finetune_human' # Subdirectory name for this run
    )

    # --- 4. Post-Training Information ---

    # The training process automatically saves the best and last model weights.
    # The path to the best model will be printed to the console at the end of training.
    # It's typically located at: 'runs/detect/yolo_finetune_human/weights/best.pt'
    print("Training complete.")
    print("The best model weights are saved in the 'runs/detect/yolo_finetune_human/weights' directory.")
    print("You can now use 'best.pt' for inference on new CCTV footage.")

if __name__ == '__main__':
    main()
