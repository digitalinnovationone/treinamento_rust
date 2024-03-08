import { Injectable } from '@angular/core';
import { CanActivate, Router } from '@angular/router';

@Injectable({
  providedIn: 'root'
})
export class AuthGuard implements CanActivate {

  constructor(private router: Router) {}

  canActivate(): boolean {
    const sessionData = sessionStorage.getItem('sessionData');
    if (sessionData) {
      const { token } = JSON.parse(sessionData);
      if (token) {
        return true; // Permitir navegação se o token existir
      }
    }
    
    // Se não houver token, redirecionar para a página de login
    this.router.navigate(['/'
    ]);
    return false;
  }
}
