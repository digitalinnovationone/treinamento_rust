import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { environment } from '../../environments/environment';
import { Recurso } from '../models/recurso';
import { firstValueFrom } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class RecursoService {
  private apiUrl = `${environment.apiHost}/recursos`;

  constructor(private http: HttpClient) {}

  private getHeaders(): HttpHeaders {
    // Obter o token do sessionStorage
    const token = sessionStorage.getItem('sessionData') ? JSON.parse(sessionStorage.getItem('sessionData')!).token : null;
    // Retornar os cabeçalhos HTTP incluindo o token de autorização, se presente
    return new HttpHeaders({
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    });
  }

  async getRecursos(): Promise<Recurso[]> {
    const headers = this.getHeaders();
    return firstValueFrom(this.http.get<Recurso[]>(this.apiUrl, { headers }));
  }

  async criarRecurso(recurso: Recurso): Promise<Recurso> {
    const headers = this.getHeaders();
    return firstValueFrom(this.http.post<Recurso>(this.apiUrl, recurso, { headers }));
  }

  async alterarRecurso(recurso: Recurso): Promise<Recurso> {
    const headers = this.getHeaders();
    return firstValueFrom(this.http.put<Recurso>(`${this.apiUrl}/${recurso.id}`, recurso, { headers }));
  }

  async excluirRecurso(id: number): Promise<void> {
    const headers = this.getHeaders();
    return firstValueFrom(this.http.delete<void>(`${this.apiUrl}/${id}`, { headers }));
  }
}
