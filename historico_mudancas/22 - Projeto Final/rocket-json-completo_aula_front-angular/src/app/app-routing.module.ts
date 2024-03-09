import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { LoginComponent } from './pages/login/login.component';
import { RecursosComponent } from './pages/recursos/recursos.component';
import { AuthGuard } from './guard/auth-guard';

const routes: Routes = [
  { path: '', component: LoginComponent },
  { path: 'recursos', component: RecursosComponent, canActivate: [AuthGuard] }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
