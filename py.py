import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext, filedialog
import subprocess
import threading
import os
import sys
import socket
import time
import tempfile
from pathlib import Path

class AetherGUI:
    def __init__(self, root):
        self.root = root
        self.root.title("Aether - VPN Client")
        self.root.geometry("800x650")
        self.root.resizable(True, True)
        
        # متغیرها
        self.process = None
        self.is_running = False
        self.auto_reconnect = False
        self.connection_attempts = 0
        self.max_attempts = 3
        self.aether_path = None
        self.monitor_thread = None
        
        self.create_widgets()
        self.find_aether_binary()
        
    def create_widgets(self):
        # استایل
        style = ttk.Style()
        style.theme_use('clam')
        
        # فریم اصلی
        main_frame = ttk.Frame(self.root, padding="10")
        main_frame.pack(fill=tk.BOTH, expand=True)
        
        # عنوان
        title_frame = ttk.Frame(main_frame)
        title_frame.pack(fill=tk.X, pady=(0, 10))
        
        title = ttk.Label(title_frame, text="🚀 Aether VPN Client", 
                         font=("Arial", 18, "bold"))
        title.pack(side=tk.LEFT)
        
        self.status_indicator = ttk.Label(title_frame, text="⚪", font=("Arial", 14))
        self.status_indicator.pack(side=tk.RIGHT)
        
        # تب‌ها
        notebook = ttk.Notebook(main_frame)
        notebook.pack(fill=tk.BOTH, expand=True, pady=10)
        
        # تب تنظیمات
        settings_frame = ttk.Frame(notebook, padding="10")
        notebook.add(settings_frame, text="⚙️ Settings")
        
        # مسیر باینری
        binary_frame = ttk.LabelFrame(settings_frame, text="Binary Location", padding="10")
        binary_frame.grid(row=0, column=0, columnspan=3, sticky=tk.W+tk.E, pady=5)
        
        self.binary_path_var = tk.StringVar(value="Not found")
        binary_entry = ttk.Entry(binary_frame, textvariable=self.binary_path_var, width=50)
        binary_entry.pack(side=tk.LEFT, fill=tk.X, expand=True)
        
        browse_btn = ttk.Button(binary_frame, text="Browse", command=self.browse_binary)
        browse_btn.pack(side=tk.RIGHT, padx=5)
        
        # پروتکل
        ttk.Label(settings_frame, text="Protocol:", font=("Arial", 10, "bold")).grid(row=1, column=0, sticky=tk.W, pady=5)
        self.protocol_var = tk.StringVar(value="masque")
        protocol_combo = ttk.Combobox(settings_frame, textvariable=self.protocol_var,
                                     values=["masque", "wg", "gool"],
                                     state="readonly", width=15)
        protocol_combo.grid(row=1, column=1, sticky=tk.W, padx=10, pady=5)
        ttk.Label(settings_frame, text="Modern QUIC-based (recommended)").grid(row=1, column=2, sticky=tk.W, padx=10)
        
        # Obfuscation
        ttk.Label(settings_frame, text="Obfuscation:", font=("Arial", 10, "bold")).grid(row=2, column=0, sticky=tk.W, pady=5)
        self.noize_var = tk.StringVar(value="balanced")
        noize_combo = ttk.Combobox(settings_frame, textvariable=self.noize_var,
                                  values=["off", "light", "balanced", "aggressive"],
                                  state="readonly", width=15)
        noize_combo.grid(row=2, column=1, sticky=tk.W, padx=10, pady=5)
        ttk.Label(settings_frame, text="Traffic obfuscation strength").grid(row=2, column=2, sticky=tk.W, padx=10)
        
        # Scan Mode
        ttk.Label(settings_frame, text="Scan Mode:", font=("Arial", 10, "bold")).grid(row=3, column=0, sticky=tk.W, pady=5)
        self.scan_var = tk.StringVar(value="turbo")
        scan_combo = ttk.Combobox(settings_frame, textvariable=self.scan_var,
                                 values=["turbo", "balanced", "thorough", "stealth"],
                                 state="readonly", width=15)
        scan_combo.grid(row=3, column=1, sticky=tk.W, padx=10, pady=5)
        ttk.Label(settings_frame, text="Scan speed vs accuracy").grid(row=3, column=2, sticky=tk.W, padx=10)
        
        # IP Version
        ttk.Label(settings_frame, text="IP Version:", font=("Arial", 10, "bold")).grid(row=4, column=0, sticky=tk.W, pady=5)
        self.ip_var = tk.StringVar(value="IPv4")
        ip_combo = ttk.Combobox(settings_frame, textvariable=self.ip_var,
                               values=["IPv4", "IPv6", "Both"],
                               state="readonly", width=15)
        ip_combo.grid(row=4, column=1, sticky=tk.W, padx=10, pady=5)
        ttk.Label(settings_frame, text="IP version to scan").grid(row=4, column=2, sticky=tk.W, padx=10)
        
        # SOCKS Port
        ttk.Label(settings_frame, text="SOCKS Port:", font=("Arial", 10, "bold")).grid(row=5, column=0, sticky=tk.W, pady=5)
        self.port_var = tk.StringVar(value="1819")
        port_entry = ttk.Entry(settings_frame, textvariable=self.port_var, width=10)
        port_entry.grid(row=5, column=1, sticky=tk.W, padx=10, pady=5)
        ttk.Label(settings_frame, text="Local proxy port (default: 1819)").grid(row=5, column=2, sticky=tk.W, padx=10)
        
        # Auto reconnect
        self.auto_reconnect_var = tk.BooleanVar(value=True)
        ttk.Checkbutton(settings_frame, text="Auto Reconnect", 
                       variable=self.auto_reconnect_var).grid(row=6, column=0, columnspan=2, sticky=tk.W, pady=10)
        ttk.Label(settings_frame, text="Automatically reconnect on failure").grid(row=6, column=2, sticky=tk.W, padx=10)
        
        # تب پیشرفته
        advanced_frame = ttk.Frame(notebook, padding="10")
        notebook.add(advanced_frame, text="🔧 Advanced")
        
        # HTTP/2 toggle
        self.h2_var = tk.BooleanVar(value=False)
        ttk.Checkbutton(advanced_frame, text="Use HTTP/2 (instead of HTTP/3)", 
                       variable=self.h2_var).grid(row=0, column=0, sticky=tk.W, pady=5)
        ttk.Label(advanced_frame, text="Useful when UDP/QUIC is blocked").grid(row=0, column=1, sticky=tk.W, padx=10)
        
        # Custom peer
        ttk.Label(advanced_frame, text="Custom Peer:", font=("Arial", 10, "bold")).grid(row=1, column=0, sticky=tk.W, pady=10)
        self.peer_var = tk.StringVar(value="")
        peer_entry = ttk.Entry(advanced_frame, textvariable=self.peer_var, width=30)
        peer_entry.grid(row=1, column=1, sticky=tk.W, padx=10, pady=10)
        ttk.Label(advanced_frame, text="Force specific endpoint (e.g., 162.159.198.1:443)").grid(row=1, column=2, sticky=tk.W, padx=10)
        
        # تب لاگ
        log_frame = ttk.Frame(notebook, padding="10")
        notebook.add(log_frame, text="📋 Log")
        
        # خروجی
        self.output_text = scrolledtext.ScrolledText(log_frame, height=20, 
                                                     font=("Consolas", 9))
        self.output_text.pack(fill=tk.BOTH, expand=True)
        
        # دکمه‌های کنترل
        btn_frame = ttk.Frame(main_frame)
        btn_frame.pack(fill=tk.X, pady=10)
        
        self.start_btn = ttk.Button(btn_frame, text="▶ Start Tunnel", 
                                   command=self.start_tunnel, width=15)
        self.start_btn.pack(side=tk.LEFT, padx=5)
        
        self.stop_btn = ttk.Button(btn_frame, text="⏹ Stop Tunnel", 
                                  command=self.stop_tunnel, state=tk.DISABLED, width=15)
        self.stop_btn.pack(side=tk.LEFT, padx=5)
        
        self.test_btn = ttk.Button(btn_frame, text="🔍 Test Connection", 
                                  command=self.test_connection, width=15)
        self.test_btn.pack(side=tk.LEFT, padx=5)
        
        self.clear_btn = ttk.Button(btn_frame, text="🗑 Clear Log", 
                                   command=self.clear_log, width=15)
        self.clear_btn.pack(side=tk.LEFT, padx=5)
        
        # وضعیت
        status_frame = ttk.Frame(main_frame)
        status_frame.pack(fill=tk.X, pady=5)
        
        self.status_var = tk.StringVar(value="🔴 Disconnected")
        status_label = ttk.Label(status_frame, textvariable=self.status_var, 
                                font=("Arial", 10))
        status_label.pack(side=tk.LEFT)
        
        self.details_var = tk.StringVar(value="Ready")
        details_label = ttk.Label(status_frame, textvariable=self.details_var, 
                                 font=("Arial", 9))
        details_label.pack(side=tk.RIGHT)
        
    def log(self, message, level="INFO"):
        """اضافه کردن پیام به خروجی"""
        import datetime
        timestamp = datetime.datetime.now().strftime("%H:%M:%S")
        
        # تنظیم آیکون بر اساس سطح
        icons = {
            "ERROR": "❌",
            "SUCCESS": "✅",
            "WARNING": "⚠️",
            "DEBUG": "🔍",
            "INFO": "ℹ️"
        }
        icon = icons.get(level, "ℹ️")
        
        formatted = f"[{timestamp}] {icon} {message}\n"
        self.output_text.insert(tk.END, formatted)
        self.output_text.see(tk.END)
        self.root.update_idletasks()
        
    def clear_log(self):
        self.output_text.delete(1.0, tk.END)
        
    def browse_binary(self):
        """انتخاب دستی باینری"""
        file_path = filedialog.askopenfilename(
            title="Select Aether Binary",
            filetypes=[("Executable files", "*.exe"), ("All files", "*.*")]
        )
        if file_path:
            self.aether_path = file_path
            self.binary_path_var.set(file_path)
            self.log(f"Binary selected: {file_path}", "SUCCESS")
            
    def find_aether_binary(self):
        """پیدا کردن باینری Aether با دیباگ کامل"""
        self.log("🔍 Searching for Aether binary...", "DEBUG")
        
        # مسیرهای احتمالی
        search_paths = []
        
        # مسیر فعلی
        current_dir = os.path.dirname(os.path.abspath(__file__))
        search_paths.append(os.path.join(current_dir, "aether"))
        search_paths.append(os.path.join(current_dir, "aether.exe"))
        search_paths.append(os.path.join(current_dir, "target", "release", "aether"))
        search_paths.append(os.path.join(current_dir, "target", "release", "aether.exe"))
        
        # مسیرهای PATH
        for path in os.environ.get("PATH", "").split(os.pathsep):
            if path.strip():
                search_paths.append(os.path.join(path, "aether"))
                search_paths.append(os.path.join(path, "aether.exe"))
        
        # مسیرهای ویندوز
        if sys.platform == "win32":
            search_paths.extend([
                os.path.expanduser("~\\aether.exe"),
                os.path.expanduser("~\\Downloads\\aether.exe"),
                "C:\\aether.exe",
                os.path.join(os.environ.get("ProgramFiles", ""), "Aether", "aether.exe"),
            ])
        
        # جستجو
        for path in search_paths:
            self.log(f"  Checking: {path}", "DEBUG")
            if os.path.exists(path):
                self.log(f"  Found: {path}", "SUCCESS")
                try:
                    # تست اجرا
                    if os.access(path, os.X_OK) or sys.platform == "win32":
                        self.aether_path = path
                        self.binary_path_var.set(path)
                        self.log(f"✅ Binary found: {path}", "SUCCESS")
                        return path
                except Exception as e:
                    self.log(f"  Error checking {path}: {e}", "DEBUG")
                    
        self.log("❌ Aether binary not found in default locations", "ERROR")
        self.log("Please use 'Browse' button to select the binary manually", "WARNING")
        self.binary_path_var.set("Not found - click Browse")
        return None
        
    def build_environment(self):
        """ساخت محیط متغیرها"""
        env = os.environ.copy()
        
        # متغیرهای اصلی
        env["AETHER_PROTOCOL"] = self.protocol_var.get()
        env["AETHER_NOIZE"] = self.noize_var.get()
        env["AETHER_SCAN"] = self.scan_var.get()
        
        ip_map = {"IPv4": "ipv4", "IPv6": "ipv6", "Both": "both"}
        env["AETHER_IP"] = ip_map.get(self.ip_var.get(), "ipv4")
        
        port = self.port_var.get().strip()
        env["AETHER_SOCKS"] = f"127.0.0.1:{port}"
        
        # متغیرهای پیشرفته
        if self.h2_var.get():
            env["AETHER_MASQUE_HTTP2"] = "1"
            
        if self.peer_var.get().strip():
            env["AETHER_PEER"] = self.peer_var.get().strip()
            
        # لاگ بیشتر برای دیباگ
        env["RUST_LOG"] = "info"
        
        return env
        
    def start_tunnel(self):
        """شروع تونل با حالت Auto"""
        if self.is_running:
            self.log("Tunnel is already running", "WARNING")
            return
            
        # پیدا کردن باینری
        if not self.aether_path or not os.path.exists(self.aether_path):
            self.log("Binary not found. Please select the binary file.", "ERROR")
            messagebox.showerror("Error", 
                "Aether binary not found!\n"
                "Please click 'Browse' and select the aether.exe file.")
            return
            
        # ساختن محیط
        env = self.build_environment()
        
        self.log("=" * 60)
        self.log("🚀 Starting Aether with AUTO mode", "SUCCESS")
        self.log("=" * 60)
        
        # نمایش تنظیمات
        self.log("📋 Configuration:", "INFO")
        for key, value in env.items():
            if key.startswith("AETHER_"):
                self.log(f"    {key}={value}", "DEBUG")
                
        self.log(f"    Binary: {self.aether_path}", "DEBUG")
        self.log("=" * 60)
        
        self.status_var.set("🟡 Connecting...")
        self.status_indicator.config(text="🟡")
        self.start_btn.config(state=tk.DISABLED)
        self.connection_attempts = 0
        
        # اجرا در ترد جداگانه
        self.thread = threading.Thread(target=self.run_aether, 
                                      args=(env,), daemon=True)
        self.thread.start()
        
    def run_aether(self, env):
        """اجرای باینری Aether با auto-reconnect"""
        while self.connection_attempts < self.max_attempts:
            self.connection_attempts += 1
            
            if self.connection_attempts > 1:
                self.root.after(0, self.log, 
                    f"🔄 Auto-reconnect attempt {self.connection_attempts}/{self.max_attempts}", 
                    "WARNING")
                time.sleep(2)
                
            try:
                # اجرای باینری
                self.log(f"🔧 Running: {self.aether_path}", "DEBUG")
                
                # تنظیمات اجرا در ویندوز
                creation_flags = 0
                if sys.platform == "win32":
                    creation_flags = subprocess.CREATE_NO_WINDOW
                
                self.process = subprocess.Popen(
                    [self.aether_path],
                    env=env,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT,
                    text=True,
                    bufsize=1,
                    creationflags=creation_flags,
                    cwd=os.path.dirname(self.aether_path) if self.aether_path else None
                )
                
                self.is_running = True
                self.root.after(0, self.update_ui_after_start)
                
                # خواندن خروجی
                connection_established = False
                for line in iter(self.process.stdout.readline, ''):
                    if line:
                        # نمایش خروجی با سطح مناسب
                        line_lower = line.lower()
                        if "error" in line_lower or "failed" in line_lower:
                            self.root.after(0, self.log, line.strip(), "ERROR")
                        elif "success" in line_lower or "established" in line_lower:
                            self.root.after(0, self.log, line.strip(), "SUCCESS")
                            connection_established = True
                        elif "warn" in line_lower:
                            self.root.after(0, self.log, line.strip(), "WARNING")
                        else:
                            self.root.after(0, self.log, line.strip(), "INFO")
                        
                        # تشخیص اتصال موفق
                        if ("proxy listening" in line_lower or 
                            "socks5" in line_lower or 
                            "listening on" in line_lower):
                            connection_established = True
                            self.root.after(0, self.on_connection_success)
                            
                self.process.stdout.close()
                exit_code = self.process.wait()
                
                self.root.after(0, self.log, f"Process exited with code: {exit_code}", "DEBUG")
                
                if exit_code != 0 and self.auto_reconnect_var.get():
                    self.root.after(0, self.log, 
                        f"Process exited with code {exit_code}, attempting reconnect...", 
                        "WARNING")
                    continue
                else:
                    break
                    
            except FileNotFoundError:
                self.root.after(0, self.log, f"File not found: {self.aether_path}", "ERROR")
                self.root.after(0, self.log, "Please check the binary path", "ERROR")
                break
                
            except PermissionError:
                self.root.after(0, self.log, "Permission denied! Try running as Administrator", "ERROR")
                break
                
            except subprocess.SubprocessError as e:
                self.root.after(0, self.log, f"Subprocess error: {e}", "ERROR")
                if not self.auto_reconnect_var.get():
                    break
                    
            except Exception as e:
                self.root.after(0, self.log, f"Unexpected error: {e}", "ERROR")
                break
                
        self.root.after(0, self.cleanup)
        
    def on_connection_success(self):
        """وقتی اتصال برقرار شد"""
        self.log("🎉 Connection established successfully!", "SUCCESS")
        self.status_var.set("🟢 Connected")
        self.status_indicator.config(text="🟢")
        self.details_var.set(f"SOCKS5: 127.0.0.1:{self.port_var.get()}")
        self.stop_btn.config(state=tk.NORMAL)
        
    def update_ui_after_start(self):
        """به‌روزرسانی UI بعد از شروع"""
        self.stop_btn.config(state=tk.NORMAL)
        self.log("🚀 Tunnel is running...", "SUCCESS")
        self.log(f"📡 SOCKS5 proxy: 127.0.0.1:{self.port_var.get()}")
        self.log("🔧 Test with: curl -x socks5h://127.0.0.1:1819 https://www.cloudflare.com/cdn-cgi/trace")
        
    def stop_tunnel(self):
        """متوقف کردن تونل"""
        if self.process:
            self.log("⏹ Stopping tunnel...", "WARNING")
            self.status_var.set("🟡 Disconnecting...")
            self.status_indicator.config(text="🟡")
            try:
                self.process.terminate()
                time.sleep(1)
                if self.process.poll() is None:
                    self.process.kill()
            except Exception as e:
                self.log(f"Error stopping: {e}", "ERROR")
            self.is_running = False
            
    def cleanup(self):
        """پاک‌سازی بعد از توقف"""
        self.is_running = False
        self.process = None
        self.start_btn.config(state=tk.NORMAL)
        self.stop_btn.config(state=tk.DISABLED)
        
        if self.connection_attempts >= self.max_attempts:
            self.status_var.set("🔴 Disconnected (max attempts)")
            self.status_indicator.config(text="🔴")
            self.log("❌ Max reconnect attempts reached", "ERROR")
        else:
            self.status_var.set("🔴 Disconnected")
            self.status_indicator.config(text="🔴")
            self.log("🛑 Tunnel stopped", "WARNING")
            
        self.details_var.set("Ready")
        
    def test_connection(self):
        """تست اتصال از طریق SOCKS5"""
        if not self.is_running:
            messagebox.showwarning("Warning", "Tunnel is not running!")
            return
            
        self.log("🔍 Testing connection through SOCKS5...", "DEBUG")
        
        def test():
            try:
                import urllib.request
                
                proxy = f"socks5h://127.0.0.1:{self.port_var.get()}"
                proxy_handler = urllib.request.ProxyHandler({
                    'http': proxy,
                    'https': proxy
                })
                
                opener = urllib.request.build_opener(proxy_handler)
                response = opener.open("https://www.cloudflare.com/cdn-cgi/trace", timeout=15)
                data = response.read().decode()
                
                self.root.after(0, lambda: self.log("✅ Connection test successful!", "SUCCESS"))
                
                # نمایش اطلاعات
                lines = data.split('\n')
                for line in lines[:10]:
                    if '=' in line:
                        self.root.after(0, lambda l=line: self.log(f"  {l}", "DEBUG"))
                        
            except ImportError:
                self.root.after(0, self.log, "⚠️ PySocks not installed. Install with: pip install PySocks", "WARNING")
            except Exception as e:
                self.root.after(0, self.log, f"❌ Connection test failed: {e}", "ERROR")
                
        threading.Thread(target=test, daemon=True).start()

if __name__ == "__main__":
    root = tk.Tk()
    app = AetherGUI(root)
    root.mainloop()