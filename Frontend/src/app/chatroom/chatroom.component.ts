import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { MessagingService } from '../services/messaging.service';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ViewChild, AfterViewInit,ElementRef, Renderer2 } from '@angular/core';
import * as CryptoJS from 'crypto-js';

@Component({
  selector: 'app-chatroom',
  templateUrl: './chatroom.component.html',
  styleUrls: ['./chatroom.component.css']
})
// export class InputClearableExample {
//   value = 'Clear me';
// }
export class ChatroomComponent implements OnInit {

  // @ViewChild('mychats') myDiv: any;
  // key = CryptoJS.enc.Hex.parse('0123456789abcdef0123456789abcdef');
  key = '0123456789abcdef';
  iv = CryptoJS.enc.Hex.parse('');

  @ViewChild('mychats')
  myDiv!: ElementRef;

  messageFormGroup: FormGroup;
  constructor(private renderer: Renderer2, private _messagingService:MessagingService, private _formBuilder: FormBuilder) { 
    this.messageFormGroup = this._formBuilder.group({
      message:[null],
    });
  }

  ngAfterViewInit() {
    this.scrollToBottom();
  }
  ngAfterViewChecked() {
    this.scrollToBottom();
  }
  
  scrollToBottom(): void {
    this.renderer.setProperty(this.myDiv.nativeElement, 'scrollTop', this.myDiv.nativeElement.scrollHeight);
  }

  author:string = "";
  user:string = "";
  queue:string = "";
  token:any


  ngOnInit(): void {

    // let key = '0123456789abcdef';
    // console.log(this.encryptUsingAES("hello a",key));
    // console.log(this.encryptUsingAES("hello yash",this.key));

    // let p = this.encryptUsingAES("U2FsdGVkX1 7 aFg0Puq9VXADXgeJ/G6lPPKRYQZsKk='",this.key);
    // console.log(this.decryptionUsingAES("U2FsdGVkX1/SnLAgZdD2GQfXK+K/3A7SMMHdiB78n10=",this.key))
    // author = me
    // user = open_chat
    // console.log(this.encryptUsingTripleDES("hi again",true))
    // G4X+qoXB3CfY0iHpTfY0hw==
    // G4X+qoXB3CfY0iHpTfY0hw==
    // console.log(this.key)
    this.token = this._messagingService.getToken();
    console.log(this.token)
    this.author = this.token.username;
    this.user = this.token.open_chat;
    this.queue = this.author+this.user;

    console.log(this.author,this.user);
    // console.log(this.author+"0000");
    if (this.author>this.user) {
      this.queue = this.user+this.author;
    }
    console.log(this.queue);
    console.log(this.message_list);

    if (this.user != "") {
      this.getMessage();

    setInterval(() => {
      // Code to be executed after 1 second
      this.getMessage();
      console.log('Delayed message');
    }, 1200);
      // this.getMessage();
      // this.getMessage();
      // this.getMessage();
      // this.getMessage();
      // this.getMessage();
    }    
  }

  message_list = [
    {
      "message" : '',
      // "sent_by" : '',
      "author" : '',
      "date_time" : '',
    },
  ]


  value = '';

  sendMessage(message : any) {

    if (message.message != "") {
      const now = new Date();
      // Get the current date in ISO format (YYYY-MM-DD)
      const currentDate = now.toISOString().slice(0, 10);
      // Get the current time in 24-hour format (HH:mm:ss)
      let currentTime = now.toLocaleTimeString('en-US', { hour12: false });
      // currentTime = currentTime.replaceAll(":","");
      // const date_time = currentTime+"_"+currentDate;
      const date_time = currentTime;
      // console.log(date_time)
      let key1 = this.author;


      let new_message_decrypted = {
        "message" : message.message,
        "author" : this.author,
        "date_time" : date_time,
      };
      let iv = this.author;

      let new_message = {
        // "message" : this.encryptUsingAES(message.message,this.key),
        "message" : message.message.trim(),
        // "author" : this.encryptUsingTripleDES(this.author.trim(),true).trim(),
        // "date_time" : this.encryptUsingTripleDES(date_time.trim(),true).trim(),
        "author" : this.author,
        // "author" : this.encryptUsingAES(this.author.trim(),this.key).trim(),
        // "date_time" : this.encryptUsingAES(date_time.trim(),this.key).trim(),
        "date_time" : this.encryptUsingAES(date_time.trim(),this.key,iv).trim(),
      };
      console.log(new_message);
      // http://127.0.0.1:8080/send?q1,m1,auth,datetime
      // let payloadString = message.message+","+this.author+","+date_time;
      let payloadString = this.queue+","+new_message.message+","+new_message.author+","+new_message.date_time;
      // let payloadString = this.queue+","+message.message+","+this.author+","+date_time;
      console.log("submit => ",payloadString);
      this._messagingService.send_messages(payloadString).subscribe(
        (res:any)=>{ 
          // console.log(res);
          if (res.status == 200) {
            this.message_list.push(new_message_decrypted);
          }
        });
      this.value = '';
      // console.log(this.message_list)
      }
  }

  getMessage() {
    let payload = {
      "username" : this.queue,
      "is_valid_user" : this.token.is_valid_user,
      "is_active_user" : this.token.is_active_user
    }
    let payloadString = JSON.stringify(payload);

    this._messagingService.get_messages(payloadString).subscribe(
      (res:any)=>{ 
        // console.log(res);
        if (res.status == 200 && res.message!="") {
          
          res.message = res.message.replaceAll("'",'"');
          res.message = "[" + res.message + "]";

          let chats = JSON.parse(res.message);
          let msg_list_length = this.message_list.length;
          // console.log(chats);
          let key1 = this.author;

          for (let i=msg_list_length-1; i<chats.length;i++) {

          let key1 = this.author;

            let message_details = {
              // "message" : this.decryptionUsingAES(chats[i].message,this.key).replaceAll('"',''),
              "message" : chats[i].message,
              // "author" : this.decryptionUsingAES(chats[i].author,this.key).replaceAll('"',''),
              "author" : chats[i].author,
              "date_time" : chats[i].date_time,
              // "date_time" : this.decryptUsingTripleDES(chats[i].date_time).replaceAll('"',''),
            };
            this.message_list.push(message_details);
          }
          console.log(this.message_list);
          // console.log("===============");

          
          // // this.getMessage();
        }
      });
  }

  
  // credits -> https://stackblitz.com/edit/encryption-decryption-triple-des-angular-wo1k7m?file=src%2Fapp%2Fapp.component.ts,src%2Fapp%2Fencryption.service.ts
  key1: any = "MTIzNDU2Nzg5MEFCQ0RFRkdISUpLTE1O";
  IV = "MTIzNDU2Nzg=";

  // ENCRYPTION USING CBC TRIPLE DES
  encryptUsingTripleDES(res: any, typeObj: boolean): string {
    const data = typeObj ? JSON.stringify(res) : res;
    const keyHex = CryptoJS.enc.Utf8.parse(this.key1);
    const iv = CryptoJS.enc.Utf8.parse(this.IV);
    const mode = CryptoJS.mode.CBC;
    const encrypted = CryptoJS.TripleDES.encrypt(data, keyHex, { iv, mode });
    return encrypted.toString();
  }

  // DECRYPTION USING CBC TRIPLE DES
  decryptUsingTripleDES(encrypted: string): string {
    const keyHex = CryptoJS.enc.Utf8.parse(this.key1);
    const iv = CryptoJS.enc.Utf8.parse(this.IV);
    const mode = CryptoJS.mode.CBC;
    const decrypted = CryptoJS.TripleDES.decrypt(encrypted, keyHex, { iv, mode });
    return decrypted.toString(CryptoJS.enc.Utf8);
  }

    
  encryptUsingAES(plaintext:any,enc_key:any,iv:any) {
    let key = CryptoJS.enc.Hex.parse(enc_key);
    // const iv = CryptoJS.enc.Hex.parse('');
    let ciphertext = CryptoJS.AES.encrypt(plaintext, this.key, { iv: this.iv, mode: CryptoJS.mode.CBC }).toString();
    ciphertext = ciphertext.replaceAll("\\",">>");
    return ciphertext;
  }

  decryptionUsingAES(ciphertext:any,dec_key:any,iv:any) {
    let key = CryptoJS.enc.Hex.parse(dec_key);
    // const iv = CryptoJS.enc.Hex.parse('');
    ciphertext = ciphertext.replaceAll(">>","\\");
    // const plaintext = CryptoJS.AES.encrypt(ciphertext, key, { iv: iv, mode: CryptoJS.mode.CBC }).toString(CryptoJS.enc.Utf8);
    const plaintext = CryptoJS.AES.decrypt(ciphertext, this.key, { iv: this.iv, mode: CryptoJS.mode.CBC }).toString(CryptoJS.enc.Utf8);
    return plaintext;
  }

}
