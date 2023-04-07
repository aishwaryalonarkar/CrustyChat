import { Injectable } from '@angular/core';
import { Subject, BehaviorSubject } from 'rxjs';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class MessagingService {

  private token = new BehaviorSubject(0);
  public token$ = this.token.asObservable();

  private message = new BehaviorSubject(0);
  public message$ = this.message.asObservable();

  setToken(is_token:any){
    this.token.next(is_token);
    sessionStorage.setItem('token',JSON.stringify(is_token));
  }

  getToken(){
    return JSON.parse(sessionStorage.getItem('token') || '{}');
  }

  setMessage(list_message:any){
    this.message.next(list_message);
    sessionStorage.setItem('message',JSON.stringify(list_message));
  }

  getMessage(){
    return JSON.parse(sessionStorage.getItem('message') || '{}');
  }

  messaging_url = "http://127.0.0.1:8081";
  user_data_url = "http://127.0.0.1:8081";
  authentication_url = "";

  constructor(private _http:HttpClient) { }

  send_messages(message_details:String){
    return this._http.get(this.messaging_url+"/send?"+message_details);
  }

  sign_in(user:String){
    return this._http.get(this.user_data_url+"/sign_in?"+user);
  }

  add_user_chat(user:String){
    return this._http.get(this.user_data_url+"/add_user_chat?"+user);
  }

  get_user_chat_list(user:String){
    return this._http.get(this.user_data_url+"/get_user_chat_list?"+user);
  }

  get_messages(user:String){
    return this._http.get(this.user_data_url+"/get_messages?"+user);
  }

}
