import serial.tools.list_ports
import time

print("Listing all serial ports...")
for port in serial.tools.list_ports.comports():
    print(f"Port: {port.device}")
    print(f"  Name: {port.name}")
    print(f"  Description: {port.description}")
    print(f"  HWID: {port.hwid}")
    print(f"  VID:PID: {port.vid}:{port.pid}")
    print()
    
    # Try to open and identify
    try:
        ser = serial.Serial(port.device, timeout=0.1)
        ser.write(b'AT\r\n')  # Simple test
        time.sleep(0.1)
        response = ser.read(100)
        if response:
            print(f"  Response: {response}")
        ser.close()
    except:
        pass