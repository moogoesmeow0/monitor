import cv2
import argparse
from ultralytics import YOLO
import csv
from datetime import datetime
import os

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

def main(model_path, video_source, output_csv):
    """
    Runs continuous inference on a video stream with a YOLO model,
    detects persons, and logs their coordinates and timestamps to a CSV file.
    """
    # Load the YOLO model
    try:
        model = YOLO(model_path)
    except Exception as e:
        print(f"Error loading model: {e}")
        print("Please ensure the model path is correct and the model is compatible with the ultralytics library.")
        return

    # Open the video source
    cap = cv2.VideoCapture(video_source)
    if not cap.isOpened():
        print(f"Error: Could not open video stream at {video_source}")
        return

    # Check if CSV file exists, if not, write header
    file_exists = os.path.isfile(output_csv)
    with open(output_csv, 'a', newline='') as csvfile:
        csv_writer = csv.writer(csvfile)
        if not file_exists:
            csv_writer.writerow(['x coord', 'y coord', 'timestamp'])

        while cap.isOpened():
            ret, frame = cap.read()
            if not ret:
                print("Stream ended or failed to read frame.")
                break

            # Apply adaptive equalization to the frame
            equalized_frame = apply_adaptive_equalization(frame)

            # Run inference on the equalized frame
            results = model.predict(equalized_frame, verbose=False)

            # Get the annotated frame
            annotated_frame = results[0].plot()
            
            # Process detections
            for r in results:
                for box in r.boxes:
                    # Check if the detected object is a person
                    if model.names[int(box.cls)] == 'person':
                        # Get bounding box coordinates
                        x1, y1, x2, y2 = box.xyxy[0]
                        
                        # Calculate center coordinates
                        center_x = (x1 + x2) / 2
                        center_y = (y1 + y2) / 2
                        
                        # Get current timestamp
                        timestamp = datetime.now()
                        
                        # Write data to CSV
                        csv_writer.writerow([int(center_x), int(center_y), timestamp])
                        csvfile.flush() # Ensure data is written immediately

            # Display the annotated frame
            cv2.imshow("Annotated Stream", annotated_frame)

            # Break the loop if 'q' is pressed
            if cv2.waitKey(1) & 0xFF == ord('q'):
                break

    # Release resources
    cap.release()
    cv2.destroyAllWindows()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run YOLO inference on a video stream with adaptive equalization and log person detections.")
    parser.add_argument("model_path", type=str, help="Path to the fine-tuned YOLO model file (e.g., 'yolov8n.pt').")
    parser.add_argument("--video_source", type=str, default="/dev/video1", help="Path to the video stream (e.g., '/dev/video1' or a video file path).")
    parser.add_argument("--output_csv", type=str, default="data.csv", help="Path to the output CSV file for observation data.")
    
    args = parser.parse_args()
    
    main(args.model_path, args.video_source, args.output_csv)
