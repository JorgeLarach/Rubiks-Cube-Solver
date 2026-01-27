import serial
import time

def send_data():
    port = '/dev/tty.usbmodem2103'  
    baudrate = 115200  
    
    data = bytes([18] * 54)
    
    try:
        ser = serial.Serial(port, baudrate, timeout=1)
        time.sleep(0.1) 
        
        ser.write(data)

        time.sleep(0.5);
        
        ser.close()
        print("Done! You can now unplug the USB cable.")
        
    except serial.SerialException as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    send_data()

