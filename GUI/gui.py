 #
 # gui.py
 #
 #  Created on: Jan 28, 2026
 #      Author: jorgelarach
 #   Co-Author: Deepseek
 #

import tkinter as tk
from tkinter import ttk, messagebox
import serial
import time

class RubiksCubeGUI:
    def __init__(self, root):
        self.root = root
        self.root.title("Rubik's Cube Solver Controller")
        self.root.geometry("1100x850")
        
        # Color mapping: characters to colors and values
        self.color_map = {
            'W': ("white", 0),
            'Y': ("yellow", 1),
            'B': ("blue", 2),
            'G': ("green", 3),
            'R': ("red", 4),
            'O': ("orange", 5)
        }
        
        # Store entry widgets for all 54 stickers
        self.sticker_entries = []
        
        # UART settings
        self.port = '/dev/tty.usbmodem2103'  # Adjust for your system
        self.baudrate = 115200
        
        self.create_widgets()
        self.update_colors()
    
    def create_widgets(self):
        # Title
        title_label = tk.Label(self.root, text="Rubik's Cube Configuration", 
                              font=("Arial", 16, "bold"))
        title_label.pack(pady=10)
        
        # Instructions
        inst_frame = tk.Frame(self.root)
        inst_frame.pack(pady=5)
        
        tk.Label(inst_frame, text="Enter color codes (W=White, Y=Yellow, B=Blue, G=Green, R=Red, O=Orange)", 
                font=("Arial", 10)).pack()
        tk.Label(inst_frame, text="Click 'Update Colors' to apply colors or 'Validate Cube' to check and color automatically",
                font=("Arial", 9), fg="gray").pack()
        
        # Main container for cube display
        main_frame = tk.Frame(self.root)
        main_frame.pack(expand=True, fill=tk.BOTH, padx=20, pady=10)
        
        # Create the traditional cross layout
        self.create_cube_display(main_frame)
        
        # Control buttons
        control_frame = tk.Frame(self.root)
        control_frame.pack(pady=20)
        
        tk.Button(control_frame, text="Update Colors", 
                 command=self.update_colors, bg="lightblue",
                 font=("Arial", 12)).pack(side=tk.LEFT, padx=10)
        
        tk.Button(control_frame, text="Validate Cube", 
                 command=self.validate_cube, bg="lightgreen",
                 font=("Arial", 12)).pack(side=tk.LEFT, padx=10)
        
        tk.Button(control_frame, text="Send via UART", 
                 command=self.send_via_uart, bg="lightcoral",
                 font=("Arial", 12)).pack(side=tk.LEFT, padx=10)
        
        tk.Button(control_frame, text="Clear All", 
                 command=self.clear_all, bg="lightgray",
                 font=("Arial", 12)).pack(side=tk.LEFT, padx=10)
        
        tk.Button(control_frame, text="Fill Solved", 
                 command=self.fill_solved, bg="lightyellow",
                 font=("Arial", 12)).pack(side=tk.LEFT, padx=10)
        
        # Status label
        self.status_label = tk.Label(self.root, text="Ready", 
                                    font=("Arial", 10), fg="gray")
        self.status_label.pack(pady=5)
        
        # Validation results
        self.validation_text = tk.Text(self.root, height=4, width=80,
                                      font=("Courier", 9))
        self.validation_text.pack(pady=10)
        
        # Color legend
        legend_frame = tk.Frame(self.root)
        legend_frame.pack(pady=10)
        
        for char, (color_name, value) in self.color_map.items():
            frame = tk.Frame(legend_frame)
            frame.pack(side=tk.LEFT, padx=5)
            tk.Label(frame, text=f"{char}:", font=("Arial", 9, "bold")).pack(side=tk.LEFT)
            tk.Label(frame, text=color_name.capitalize(), font=("Arial", 9), 
                    bg=color_name, width=8).pack(side=tk.LEFT)
        
    def create_cube_display(self, parent):
        # Create a canvas for the traditional cross layout
        canvas_width = 1000
        canvas_height = 600
        
        self.canvas = tk.Canvas(parent, width=canvas_width, 
                               height=canvas_height, bg="lightgray")
        self.canvas.pack()
        
        # Define face positions (x, y) for each face in cross pattern
        face_positions = {
            'U': (400, 50),   # Top
            'D': (400, 350),   # Bottom
            'L': (200, 200),  # Left
            'R': (600, 200),  # Right
            'F': (400, 200),  # Center
            'B': (800, 200)   # Far right
        }
        
        face_size = 100  # Size of each 3x3 grid
        sticker_size = 30  # Size of each individual sticker
        
        # Create entry boxes for each face
        for face_name, (x, y) in face_positions.items():
            # Draw face label
            self.canvas.create_text(x + face_size//2, y - 30, 
                                   text=face_name, font=("Arial", 14, "bold"), fill='black')
            
            # Draw a background for the face
            # self.canvas.create_rectangle(
            #     x - 5, y - 5,
            #     x + face_size + 5, y + face_size + 5,
            #     fill="black", outline="black", width=2
            # )
            
            # Create 3x3 grid of entry boxes for this face
            for row in range(3):
                for col in range(3):
                    # Calculate position
                    sticker_x = x + col * (sticker_size + 2)
                    sticker_y = y + row * (sticker_size + 2)
                    
                    # Create an entry widget
                    entry_var = tk.StringVar(value="W")
                    entry = tk.Entry(self.canvas, textvariable=entry_var, 
                                    width=2, font=("Arial", 10, "bold"),
                                    justify="center", bg="white")
                    
                    # Place the entry on canvas
                    self.canvas.create_window(sticker_x + sticker_size//2, 
                                            sticker_y + sticker_size//2,
                                            window=entry, width=sticker_size,
                                            height=sticker_size)
                    
                    # Draw a border around the sticker
                    self.canvas.create_rectangle(
                        sticker_x, sticker_y,
                        sticker_x + sticker_size, sticker_y + sticker_size,
                        outline="black", width=1
                    )
                    
                    self.sticker_entries.append(entry)
    
    def update_colors(self):
        """Update the background colors of all stickers based on entered characters"""
        for entry in self.sticker_entries:
            char = entry.get().strip().upper()
            if char and char in self.color_map:
                color_name, _ = self.color_map[char]
                entry.config(bg=color_name)
                # Make text white for dark backgrounds
                if color_name in ["blue", "green", "red", "orange"]:
                    entry.config(fg="white")
                else:
                    entry.config(fg="black")
            else:
                entry.config(bg="white", fg="black")
        self.status_label.config(text="Colors updated", fg="blue")
    
    def get_cube_state_bytes(self):
        """Convert all 54 entries to a bytes object (0-5 values)"""
        byte_list = []
        invalid_entries = []
        
        for i, entry in enumerate(self.sticker_entries):
            char = entry.get().strip().upper()
            if char in self.color_map:
                _, value = self.color_map[char]
                byte_list.append(value)
            else:
                invalid_entries.append(i)
                byte_list.append(0)  # Default to white if invalid
        
        # Highlight invalid entries
        for i in invalid_entries:
            self.sticker_entries[i].config(bg="pink")
        
        # Ensure we have exactly 54 bytes
        if len(byte_list) != 54:
            messagebox.showerror("Error", f"Expected 54 stickers, got {len(byte_list)}")
            return None
        
        return bytes(byte_list)
    
    def validate_cube(self):
        """Validate the cube configuration and update colors"""
        # First update colors
        self.update_colors()
        
        cube_bytes = self.get_cube_state_bytes()
        if cube_bytes is None:
            return False
        
        validation_results = []
        
        # 1. Check for invalid characters
        invalid_count = 0
        for entry in self.sticker_entries:
            char = entry.get().strip().upper()
            if not char or char not in self.color_map:
                invalid_count += 1
        
        if invalid_count > 0:
            validation_results.append(f"⚠ {invalid_count} stickers have invalid colors")
        
        # 2. Check we have exactly 9 of each color
        color_counts = [0] * 6
        for byte in cube_bytes:
            color_counts[byte] += 1
        
        color_names = ["White", "Yellow", "Blue", "Green", "Red", "Orange"]
        valid = True
        
        for i, count in enumerate(color_counts):
            color_name = color_names[i]
            if count != 9:
                valid = False
                validation_results.append(f"✗ {color_name}: {count}/9 stickers")
            else:
                validation_results.append(f"✓ {color_name}: {count}/9 stickers")
        
        # 3. Check center pieces uniqueness
        center_indices = [4, 13, 22, 31, 40, 49]  # Centers of each face
        center_colors = [cube_bytes[i] for i in center_indices]
        if len(set(center_colors)) != 6:
            valid = False
            validation_results.append("✗ Center pieces not unique (each face should have a different center)")
        else:
            validation_results.append("✓ Center pieces are unique")
        
        # 4. Quick edge check (simplified)
        # Check that opposite centers are actually opposite colors
        # White opposite Yellow, Blue opposite Green, Red opposite Orange
        opposite_pairs = [(0, 1), (2, 3), (4, 5)]
        face_centers = {
            'U': cube_bytes[4],    # White
            'D': cube_bytes[13],   # Yellow
            'F': cube_bytes[22],   # Blue
            'B': cube_bytes[31],   # Green
            'R': cube_bytes[40],   # Red
            'L': cube_bytes[49]    # Orange
        }
        
        # Map face names to indices for checking
        face_map = {'U': 0, 'D': 1, 'F': 2, 'B': 3, 'R': 4, 'L': 5}
        for color1, color2 in opposite_pairs:
            # Find which faces have these colors as centers
            faces1 = [face for face, color in face_centers.items() if color == color1]
            faces2 = [face for face, color in face_centers.items() if color == color2]
            
            if faces1 and faces2:
                # These should be opposite faces
                validation_results.append(f"✓ {color_names[color1]} opposite {color_names[color2]}")
            else:
                validation_results.append(f"⚠ Missing opposite pair: {color_names[color1]}/{color_names[color2]}")
        
        # Display results
        self.validation_text.delete(1.0, tk.END)
        for result in validation_results:
            self.validation_text.insert(tk.END, result + "\n")
        
        if valid and invalid_count == 0:
            self.status_label.config(text="Cube is valid!", fg="green")
        else:
            self.status_label.config(text="Cube has issues", fg="red")
        
        return valid
    
    def send_via_uart(self):
        """Send the cube state via UART"""
        if not self.validate_cube():
            messagebox.showwarning("Validation Failed", 
                                 "Fix cube issues before sending")
            return
        
        cube_bytes = self.get_cube_state_bytes()
        if cube_bytes is None:
            return
        
        print(cube_bytes)

        try:
            # Open serial connection
            ser = serial.Serial(self.port, self.baudrate, timeout=2)
            time.sleep(2)  # Wait for connection
            
            # Send the 54 bytes
            ser.write(cube_bytes)
            
            # Wait for response
            time.sleep(0.5)
            responses = []
            while ser.in_waiting > 0:
                response = ser.readline().decode('utf-8', errors='ignore').strip()
                if response:
                    responses.append(response)
            
            ser.close()
            
            # Show results
            if responses:
                response_text = "\n".join(responses)
                messagebox.showinfo("Success", 
                                  f"Data sent successfully!\n\nSTM32 responded:\n{response_text}")
            else:
                messagebox.showinfo("Success", "Data sent successfully!")
                
            self.status_label.config(text=f"Sent {len(cube_bytes)} bytes", fg="blue")
            
        except serial.SerialException as e:
            messagebox.showerror("UART Error", 
                               f"Failed to connect to {self.port}:\n{str(e)}")
            self.status_label.config(text="UART connection failed", fg="red")
        except Exception as e:
            messagebox.showerror("Error", f"Unexpected error: {str(e)}")
    
    def clear_all(self):
        """Reset all stickers to white (W)"""
        for entry in self.sticker_entries:
            entry.delete(0, tk.END)
            entry.insert(0, "W")
            entry.config(bg="white", fg="black")
        self.validation_text.delete(1.0, tk.END)
        self.status_label.config(text="All stickers cleared to white", fg="gray")
    
    def fill_solved(self):
        """Fill with a solved cube state for testing"""
        # Solved cube mapping (standard colors)
        solved_mapping = {
            'U': 'W',  # White
            'D': 'Y',  # Yellow
            'F': 'B',  # Blue
            'B': 'G',  # Green
            'R': 'R',  # Red
            'L': 'O'   # Orange
        }
        
        # Face order in our display: U, L, F, R, B, D
        face_order = ['U', 'L', 'F', 'R', 'B', 'D']
        
        for face_idx, face_name in enumerate(face_order):
            color_char = solved_mapping[face_name]
            # Each face has 9 stickers
            for i in range(9):
                entry_idx = face_idx * 9 + i
                self.sticker_entries[entry_idx].delete(0, tk.END)
                self.sticker_entries[entry_idx].insert(0, color_char)
        
        self.update_colors()
        self.status_label.config(text="Solved cube loaded", fg="green")

def main():
    root = tk.Tk()
    app = RubiksCubeGUI(root)
    root.mainloop()

if __name__ == "__main__":
    main()