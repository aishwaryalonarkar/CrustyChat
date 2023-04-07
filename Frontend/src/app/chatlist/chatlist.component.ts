import { Component, OnInit } from '@angular/core';
import { FormBuilder,FormGroup } from '@angular/forms';
import { Routes, RouterModule } from '@angular/router';
import { ChatroomComponent } from '../chatroom/chatroom.component';
import { MessagingService } from '../services/messaging.service';
import { Router } from '@angular/router';
import { Location } from '@angular/common';

const routes: Routes = [
  { path: '/chat', component: ChatroomComponent },
];

@Component({
  selector: 'app-chatlist',
  templateUrl: './chatlist.component.html',
  styleUrls: ['./chatlist.component.css']
})
export class ChatlistComponent implements OnInit {

  chatList = [];
  username = "";
  addUserForm: FormGroup;
  value = "";
  token:any;

  constructor(private location: Location, private _messagingService : MessagingService, private _formBuilder: FormBuilder, private router: Router) { 
    this.addUserForm = this._formBuilder.group({
      username:[null],
    });
  }

  ngOnInit(): void {
    this.token = this._messagingService.getToken();
    this.chatList = this.token.chats;
    this.username = this.token.username;
    console.log(this.chatList,this.username);
  }

  addUser(username : any) {
    console.log(username);
    
    let username_string:String = username.username;
    this.value = "";

    let new_user = {
      "username" : username_string,
      "user" : this.username,
    };
    if (!this.chatList.includes(username_string as never) && username_string!=this.username){
      // http://127.0.0.1:8080/send?q1,m1,auth,datetime
      let payloadString = JSON.stringify(new_user);
      console.log(payloadString);
      console.log("submit => ",payloadString);
      this._messagingService.add_user_chat(payloadString).subscribe(
        (res:any)=>{ 
          console.log(res);
          if (res.status == 200) {
            if (res.status == 200 && res.message=="Success") {
              this.refresh_chatList();
            }
          }
        });
    }
  }

  refresh_chatList() {
    let payload = {
      "username" : this.token.username,
      "is_valid_user" : this.token.is_valid_user,
      "is_active_user" : this.token.is_active_user
    }
    // console.log(payload);
    let payloadString = JSON.stringify(payload);
    this._messagingService.get_user_chat_list(payloadString).subscribe((response:any) => {
      console.log(response);
      if (response.status == 200) {
        let msg = response.message.split(",");
        let status_message = msg[0];
        if (status_message == "Success") {
          const chats = msg.slice(1, msg.length + 1);   
          let token = {
              "username" : this.token.username,
              "chats" : chats,
              "is_valid_user" : true,
              "is_active_user" : true
          }
          // console.log(token);
          this._messagingService.setToken(token);
          // this.router.navigate(['/chatList'])   
          // location.reload(); 
          this.token = this._messagingService.getToken();
          this.chatList = this.token.chats;
        }
      }
    });
    // if 
  }

  change_open_chat(user:any) {
    console.log(user);
    this.token.open_chat = user.user;
    this._messagingService.setToken(this.token);
    location.reload();
  }
}
