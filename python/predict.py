from ultralytics import YOLO
import cv2
import os

def adaptive_equalization(img):
    """Apply CLAHE (adaptive equalization) to a color or grayscale image."""
    # Convert to LAB color space (better for contrast adjustment)
    lab = cv2.cvtColor(img, cv2.COLOR_BGR2LAB)
    l, a, b = cv2.split(lab)

    # Apply CLAHE to the L-channel
    clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8,8))
    cl = clahe.apply(l)

    # Merge the channels and convert back to BGR
    merged = cv2.merge((cl, a, b))
    eq_img = cv2.cvtColor(merged, cv2.COLOR_LAB2BGR)
    return eq_img

def run_inference():
    # Path to fine-tuned model
    model_path = os.path.join('runs', 'detect', 'yolo_finetune_human2', 'weights', 'best.pt')
    if not os.path.exists(model_path):
        print(f"Error: Model file not found at {model_path}")
        return
    model = YOLO(model_path)

    # Path to the test image (CHANGE THIS)
    image_to_test = 'cropped.png' # <-- CHANGE THIS
    if not os.path.exists(image_to_test):
        print(f"Error: Test image not found at {image_to_test}")
        return

    # Load and apply adaptive equalization
    img = cv2.imread(image_to_test)
    eq_img = adaptive_equalization(img)

    # Optionally, save the processed image for inspection
    eq_path = "equalized_temp.jpg"
    cv2.imwrite(eq_path, eq_img)

    print(f"Running inference on adaptive equalized image: {eq_path}")
    results = model(eq_path)

    for r in results:
        print("--- Detections ---")
        for box in r.boxes:
            class_id = int(box.cls)
            class_name = model.names[class_id]
            confidence = float(box.conf)
            print(f"Detected: '{class_name}' with confidence {confidence:.2f}")
        print("--------------------")
        im_array = r.plot()
        cv2.imshow("YOLOv11 Fine-tuned Inference (Adaptive Equalization)", im_array)
        print("\nPress any key to close the image window.")
        cv2.waitKey(0)
        cv2.destroyAllWindows()

if __name__ == '__main__':
    run_inference()
