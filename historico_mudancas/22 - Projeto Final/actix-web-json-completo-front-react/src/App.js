import './App.css';
import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Clientes from './Pages/Clientes';
import Login from './Pages/Login';

function Main() {
    return (
        <div className="App">
            <Routes>
                <Route path="/" element={<Login />} />
                <Route path="/clientes" element={<Clientes />} />
            </Routes>
        </div>
    );
}

export default Main;
