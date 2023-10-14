// lib
import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter as Router } from "react-router-dom";

// components
import Navbar from "./components/Navbar.tsx";
import Footer from "./components/Footer.tsx";
import App from "./App.tsx";

// styles
import "./styles/index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <Router>
      <Navbar />
      <App />
      <Footer />
    </Router>
  </React.StrictMode>
);
