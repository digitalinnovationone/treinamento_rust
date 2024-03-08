import { Component, OnInit } from '@angular/core';
import { Recurso } from '../../models/recurso';
import { RecursoService } from '../../services/recurso-service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-recursos',
  templateUrl: './recursos.component.html',
  styleUrls: ['./recursos.component.css']
})
export class RecursosComponent implements OnInit {
  recursos: Recurso[] = [];
  mostrarFormulario: boolean = false;
  recursoAtual: Recurso = {
    id: 0,
    titulo: '',
    descricao: ''
  };

  constructor(private recursoService: RecursoService, private router: Router) {}

  async ngOnInit(): Promise<void> {
    await this.buscarRecursos();
  }

  async buscarRecursos(): Promise<void> {
    this.recursos = await this.recursoService.getRecursos();
  }

  async excluirRecurso(id: number): Promise<void> {
    if(window.confirm("Confirma exclusão?")){
      await this.recursoService.excluirRecurso(id);
      await this.buscarRecursos();
    }
  }

  mostrarNovoFormulario() {
    this.recursoAtual = { id: 0, titulo: '', descricao: '' }; // Reset ou criar novo objeto
    this.mostrarFormulario = true;
  }
  
  editarRecurso(recurso: Recurso) {
    this.recursoAtual = { ...recurso }; // Copiar o objeto para edição
    this.mostrarFormulario = true;
  }
  
  async salvarRecurso() {
    if (this.recursoAtual) {
      // Verificar se é um novo recurso ou uma atualização
      if (this.recursoAtual.id === 0) {
        // Novo recurso
        try {
          const novoRecurso = await this.recursoService.criarRecurso(this.recursoAtual);
          console.log('Recurso criado:', novoRecurso);
        } catch (error) {
          console.error('Erro ao criar recurso:', error);
        }
      } else {
        // Atualizar recurso existente
        try {
          const recursoAtualizado = await this.recursoService.alterarRecurso(this.recursoAtual);
          console.log('Recurso atualizado:', recursoAtualizado);
        } catch (error) {
          console.error('Erro ao atualizar recurso:', error);
        }
      }
  
      // Após salvar ou atualizar, ocultar o formulário e atualizar a lista de recursos
      this.mostrarFormulario = false;
      await this.buscarRecursos();
    }
  }

  sair() {
    window.sessionStorage.clear()
    this.router.navigate(['/']);
  }
}
