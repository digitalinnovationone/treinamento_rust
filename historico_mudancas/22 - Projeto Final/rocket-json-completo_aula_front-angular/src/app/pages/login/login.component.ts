import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { environment } from '../../../environments/environment'

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {
  email: string;
  password: string;
  errorMessage: string; // Variável para armazenar mensagens de erro

  constructor(private http: HttpClient, private router: Router) { 
    this.email = "";
    this.password = "";
    this.errorMessage = "";
  }

  ngOnInit(): void {
  }

  login(): void {
    const loginData = { email: this.email, senha: this.password };
    this.http.post(`${environment.apiHost}/login`, loginData).subscribe({
      next: (response:any) => {
        
        const token = response.token;
        const now = new Date();
        const expirationDate = new Date(now.getTime() + 24 * 60 * 60 * 1000); // Definir a expiração para 1 dia
        const sessionData = { token: token, expiration: expirationDate.toISOString() };
        sessionStorage.setItem('sessionData', JSON.stringify(sessionData));

        this.router.navigate(['/recursos']);
      },
      error: (error) => {
        this.errorMessage = 'Login ou senha inválidos';
      }
    });
  }
}
