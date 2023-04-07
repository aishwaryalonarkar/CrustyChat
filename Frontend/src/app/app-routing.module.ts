import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { LoginComponent  } from './login/login.component';
import { RegisterComponent } from './register/register.component';
import { FailedComponent } from './failed/failed.component';
import { ChatlistComponent } from './chatlist/chatlist.component';
import { ChatroomComponent } from './chatroom/chatroom.component';

const routes: Routes = [
  { path: '', component: LoginComponent },
  { path: 'chats', component: ChatroomComponent },
  { path: 'chatList', component: ChatlistComponent },
  { path: 'sign_in', component: LoginComponent },
  { path: 'sign_up', component: RegisterComponent },
  { path: 'failed', component: FailedComponent },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})

export class AppRoutingModule { }
