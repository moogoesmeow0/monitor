import cv2
import argparse
from ultralytics import YOLO

def apply_adaptive_equalization(image):
    """
    Applies Contrast Limited Adaptive Histogram Equalization (CLAHE) to the image.
    The image is converted to the YUV color space to apply equalization only on the luminance channel.
    """
    # Convert the image from BGR to YUV color space
    yuv_image = cv2.cvtColor(image, cv2.COLOR_BGR2YUV)

    # Split the YUV image into its Y, U, and V channels
    y, u, v = cv2.split(yuv_image)

    # Create a CLAHE object. clipLimit and tileGridSize are key parameters to tune.
    clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))

    # Apply CLAHE to the Y channel (the luminance channel)
    y_clahe = clahe.apply(y)

    # Merge the CLAHE enhanced Y channel back with the original U and V channels
    yuv_clahe_image = cv2.merge([y_clahe, u, v])

    # Convert the YUV image back to BGR color space
    bgr_clahe_image = cv2.cvtColor(yuv_clahe_image, cv2.COLOR_YUV2BGR)

    return bgr_clahe_image

def main(model_path, image_path):
    """
    Runs inference on an image with a YOLO model after applying adaptive equalization.
    """
    # Load the YOLO model. 
    # Note: "yolov11" is not an official YOLO model name as of my last update.
    # I am using 'yolov8n.pt' as a placeholder. 
    # Please replace 'yolov8n.pt' with the path to your fine-tuned model.
    try:
        model = YOLO(model_path)
    except Exception as e:
        print(f"Error loading model: {e}")
        print("Please ensure the model path is correct and the model is compatible with the ultralytics library.")
        return

    # Read the input image
    image = cv2.imread(image_path)
    if image is None:
        print(f"Error: Could not read image at {image_path}")
        return

    # Apply adaptive equalization to the image
    equalized_image = apply_adaptive_equalization(image)

    # Run inference on the equalized image
    results = model.predict(equalized_image)

    # Render the results on the image
    # The 'plot' method returns a new image with bounding boxes and labels drawn on it.
    annotated_image = results[0].plot()

    # Display the original, equalized, and annotated images
    cv2.imshow("Original Image", image)
    cv2.imshow("Equalized Image", equalized_image)
    cv2.imshow("Annotated Image", annotated_image)

    # Wait for a key press and then close all windows
    cv2.waitKey(0)
    cv2.destroyAllWindows()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run YOLO inference on an image with adaptive equalization.")
    parser.add_argument("model_path", type=str, help="Path to the fine-tuned YOLO model file (e.g., 'yolov8n.pt').")
    parser.add_argument("image_path", type=str, help="Path to the input image file.")
    
    args = parser.parse_args()
    
    main(args.model_path, args.image_path)
