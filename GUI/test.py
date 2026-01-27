# import serial
# import time

# def send_54_bytes():
#     # Replace 'COM3' with your actual port (COM# on Windows, /dev/ttyACM# on Linux/Mac)
#     # Common port names:
#     # - Windows: COM3, COM4, etc.
#     # - Linux: /dev/ttyACM0, /dev/ttyUSB0
#     # - Mac: /dev/tty.usbmodemXXXX
#     port = '/dev/tty.usbmodem1103'  
#     baudrate = 115200  # Default for Nucleo boards
    
#     # The 54 bytes you want to send (example: 54 zeros)
#     data_to_send = bytes([6] * 54)  # Replace with your actual 54 bytes
    
#     try:
#         # Open serial connection
#         ser = serial.Serial(port, baudrate, timeout=1)
#         time.sleep(0.1)  # Brief delay for connection to establish
        
#         # Send the 54 bytes
#         ser.write(data_to_send)
#         print(f"Sent {len(data_to_send)} bytes: {data_to_send.hex()}")
        
#         # Optional: Wait for acknowledgment (if STM32 sends one)
#         # response = ser.read(1)
#         # print(f"Response: {response}")
        
#         ser.close()
#         print("Done! You can now unplug the USB cable.")
        
#     except serial.SerialException as e:
#         print(f"Error: {e}")
#         print("\nTroubleshooting tips:")
#         print("1. Check the COM port name in Device Manager (Windows) or ls /dev/tty* (Linux/Mac)")
#         print("2. Make sure no other program (like Arduino IDE, Putty) is using the port")
#         print("3. Try unplugging and replugging the USB cable")

# if __name__ == "__main__":
#     send_54_bytes()

import serial
import time

ser = serial.Serial("/dev/tty.usbmodem1103", 115200, timeout=1)
time.sleep(2)
ser.write(b"Hello")
ser.close()