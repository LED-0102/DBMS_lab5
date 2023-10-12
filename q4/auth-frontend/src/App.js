import React from 'react';
import {BrowserRouter as Router, Routes, Route, Navigate} from 'react-router-dom';
import Register from './components/Register';
import Login from './components/Login';
import Home from "./components/Home";
import Welcome from "./components/Welcome";

const App = () => {
    return (
        <Router>
            <div className="App">
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/register" element={<Register />} />
                    <Route path="/login" element={<Login />} />
                    <Route path="/welcome" element={<Welcome />} />
                    {/* Add more routes for other components/pages */}
                </Routes>
            </div>
        </Router>
    );
};

export default App;
