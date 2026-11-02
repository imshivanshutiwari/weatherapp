# AeroCast 🌪️

**AeroCast** is a high-performance, real-time meteorological visualization platform and weather forecast application. It delivers interactive animated wind streamlines, multi-model weather projections, and high-resolution doppler radar & satellite layers.

---

## 🌟 Key Features

* **Real-time WebGL Wind Particle Simulation:** Renders live global atmospheric airflow and vector dynamics at 60 FPS using hardware-accelerated shaders.
* **10-Day Extended Forecast:** Complete multi-day meteorological forecasts with 1-hour temporal resolution.
* **Multi-Model Support:** Seamlessly inspect forecasts from ECMWF, GFS, ICON, and METEOBLUE models.
* **Doppler Radar & Satellite Imagery:** Interactive precipitation radar and infrared satellite cloud layers with full zoom capability.
* **Comprehensive Overlays:** Wind gusts, temperature, atmospheric pressure, wave swells, and Air Quality Index (AQI).
* **Cross-Platform Architecture:** Built on modern Web standards (HTML5/CSS3/ES Modules) and packaged for mobile devices with Capacitor.

---

## 🏗️ Project Architecture

```
AeroCast/
├── server.py               # Lightweight CORS proxy server for weather layers
├── extracted/
│   ├── assets/
│   │   ├── capacitor.config.json   # Mobile build configuration (com.shiva.aerocast)
│   │   └── public/                 # Web application & map interface
│   │       ├── index.html          # Application entrypoint
│   │       ├── img/                # Custom brand logos, icons, and textures
│   │       └── v/                  # WebGL shaders, Leaflet-GL engine, and plugins
└── README.md
```

---

## 🚀 Getting Started

### Prerequisites
* Python 3.8+ (for running the local server)
* Modern web browser (Chrome, Edge, Firefox, or Safari) with WebGL support

### Running Locally

1. **Clone the repository:**
   ```bash
   git clone https://github.com/imshivanshutiwari/weatherapp.git
   cd weatherapp
   ```

2. **Start the local server:**
   ```bash
   python server.py
   ```

3. **Open in your browser:**
   Navigate to [http://localhost:8080](http://localhost:8080).

---

## 📱 Mobile Build (Android)
The project is configured for Capacitor Android deployment:
```bash
npx cap sync android
npx cap open android
```

---

## 📄 License
This project is for educational and research purposes.
