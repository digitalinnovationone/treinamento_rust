import React, { useEffect, useState } from 'react';
import Env from '../Environment/Env';
import '../App.css';
import { useNavigate } from 'react-router-dom';

function Clientes() {
    const [clientes, setClientes] = useState([]);
    const [exibirForm, setExibirForm] = useState(false);
    const [clienteAtual, setClienteAtual] = useState({ id: '', nome: '', cpf: '' });
    const navigate = useNavigate();

    const fetchClientes = async () => {
        try {
            // Obter o token armazenado no sessionStorage
            const storedToken = JSON.parse(sessionStorage.getItem('authToken'));
            if (!storedToken) {
                throw new Error('Token de autenticação não encontrado');
            }
    
            const token = storedToken.value;
            const response = await fetch(`${Env.host}/clientes`, {
                method: 'GET', // Método HTTP
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}` // Passar o token no cabeçalho Authorization
                }
            });
    
            if (!response.ok) {
                throw new Error('Erro ao buscar dados dos clientes');
            }
    
            const data = await response.json();
            setClientes(data);
        } catch (error) {
            console.error(error.message);
        }
    };

    useEffect(() => {
        const storedToken = JSON.parse(sessionStorage.getItem('authToken'));
        if (!storedToken) {
            navigate('/'); // Redireciona para a tela de login
        } else {
            fetchClientes();
        }
    }, [navigate]);

    const deleteCliente = async (id) => {
        if(window.confirm('confirma exclusão ?')){
            try {
                // Obter o token armazenado no sessionStorage
                const storedToken = JSON.parse(sessionStorage.getItem('authToken'));
                if (!storedToken) {
                    throw new Error('Token de autenticação não encontrado');
                }
        
                const token = storedToken.value;
                const response = await fetch(`${Env.host}/clientes/${id}`, {
                    method: 'DELETE', // Método HTTP para deletar
                    headers: {
                        'Authorization': `Bearer ${token}` // Passar o token no cabeçalho Authorization
                    }
                });
        
                if (!response.ok) {
                    throw new Error('Erro ao excluir o cliente');
                }
        
                // Remover o cliente excluído da lista
                setClientes(clientes.filter(cliente => cliente.id !== id));
        
            } catch (error) {
                console.error(error.message);
            }
        }
    };

    const sair = () => {
        sessionStorage.clear();
        navigate('/');
    }

    const handleFormSubmit = async (event) => {
        event.preventDefault();
    
        const storedToken = JSON.parse(sessionStorage.getItem('authToken'));
        if (!storedToken) {
            console.error('Token de autenticação não encontrado');
            return;
        }
    
        const token = storedToken.value;
        const payload = {
            id: 0,
            nome: clienteAtual.nome,
            cpf: clienteAtual.cpf
        }

        let url = `${Env.host}/clientes`;
        let method = 'POST';
        if(clienteAtual.id) {
            payload.id = clienteAtual.id;
            url = `${Env.host}/clientes/${clienteAtual.id}`;
            method = 'PUT';
        }
    
        try {
            const response = await fetch(url, {
                method: method,
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                },
                body: JSON.stringify(payload)
            });
    
            if (!response.ok) {
                throw new Error('Erro ao processar a requisição');
            }
    
            // Atualizar a lista de clientes após a operação
            await fetchClientes();
            hideForm(); // Esconde o formulário após a operação bem-sucedida
        } catch (error) {
            console.error(error.message);
        }
    };
    

    const showForm = (cliente = { id: '', nome: '', cpf: '' }) => {
        setClienteAtual(cliente);
        setExibirForm(true);
    };

    const hideForm = () => {
        setExibirForm(false);
    };

    return (
        <div>
            <br/>
            <h1>Clientes</h1>
            {exibirForm && (
                <div>
                    <h2>{clienteAtual.id ? 'Editar Cliente' : 'Novo Cliente'}</h2>
                    <form className="cliente" onSubmit={handleFormSubmit}>
                        <div className="form-group">
                            <label htmlFor="nome">Nome</label>
                            <input type="text" className="form-control" id="nome" value={clienteAtual.nome} onChange={e => setClienteAtual({...clienteAtual, nome: e.target.value})} />
                        </div>
                        <div className="form-group">
                            <label htmlFor="cpf">CPF</label>
                            <input type="text" maxLength="14" className="form-control" id="cpf" value={clienteAtual.cpf} onChange={e => setClienteAtual({...clienteAtual, cpf: e.target.value})} />
                        </div>
                        <button type="submit" className="btn btn-primary">Salvar</button>
                        <button type="button" className="btn btn-secondary" onClick={hideForm}>Cancelar</button>
                    </form>
                </div>
            )}
            
            {!exibirForm && (
                <>
                    <button type="button" className="btn btn-success mb-2" onClick={() => showForm()}>Novo Cliente</button>
                    <div className="table-responsive">
                        <table className="table table-striped">
                            <thead>
                                <tr>
                                    <th>ID</th>
                                    <th>Nome</th>
                                    <th>CPF</th>
                                    <th>Ações</th>
                                </tr>
                            </thead>
                            <tbody>
                                {clientes.map(cliente => (
                                    <tr key={cliente.id}>
                                        <td>{cliente.id}</td>
                                        <td>{cliente.nome}</td>
                                        <td>{cliente.cpf}</td>
                                        <td className='table-actions'>
                                            <button type="button" className="btn btn-primary mr-2"  onClick={() => showForm(cliente)}>Alterar</button>
                                            <button type="button" className="btn btn-danger" onClick={() => deleteCliente(cliente.id)}>Excluir</button>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </>
            )}


            <br/>
            <button type="button" className="btn btn-danger mr-2"  onClick={sair}>Sair</button>
        </div>
    );
}

export default Clientes;
