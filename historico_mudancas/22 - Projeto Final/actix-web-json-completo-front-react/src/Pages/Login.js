import '../App.css';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import  Env from '../Environment/Env';

function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleLogin = async (event) => {
    event.preventDefault();

    const loginData = {
      email: email,
      senha: password,
    };

    try {
      const response = await fetch(`${Env.host}/logar`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(loginData),
      });

      if (response.ok) {
        const { token } = await response.json(); // Desestruturando para obter o token
        // Armazenar o token no sessionStorage com um tempo de expiração
        const now = new Date();
        const item = {
          value: token,
          expiry: now.getTime() + (24 * 60 * 60 * 1000), // 24 horas de validade
        };
        sessionStorage.setItem('authToken', JSON.stringify(item));
        navigate('/clientes');
      } else {
        throw new Error('Login ou senha inválidos');
      }
    } catch (error) {
      setError(error.message);
    }
  };

  return (
    <div className="App">
      <div className="login-container">
        <form className="form-signin" onSubmit={handleLogin}>
          <h1 className="h3 mb-3 font-weight-normal">Dados para entrar no sistema</h1>
          {error && <div className="alert alert-danger" role="alert">{error}</div>}
          <label htmlFor="inputEmail" className="sr-only">Endereço de email</label>
          <input
            type="email"
            id="inputEmail"
            className="form-control"
            placeholder="Endereço de email"
            required
            autoFocus
            value={email}
            onChange={e => setEmail(e.target.value)}
          />
          <label htmlFor="inputPassword" className="sr-only">Senha</label>
          <input
            type="password"
            id="inputPassword"
            className="form-control"
            placeholder="Senha"
            required
            value={password}
            onChange={e => setPassword(e.target.value)}
          />
          <button className="btn btn-lg btn-primary btn-block" type="submit">Logar</button>
        </form>
      </div>
    </div>
  );
}

export default Login;
