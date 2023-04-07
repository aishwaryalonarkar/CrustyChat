import { Component, OnInit } from '@angular/core';
import {FormBuilder, FormGroup, Validators} from '@angular/forms';
import { Router } from '@angular/router';
import {MatSnackBar} from '@angular/material/snack-bar';
import { HttpClient } from '@angular/common/http';
import { MessagingService } from '../services/messaging.service';

import * as CryptoJS from 'crypto-js';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {

  firstFormGroup: FormGroup;
  is_auth = {
    "auth1" : false, // login
    "auth2" : false, // otp
    "auth3" : false, // face 
    "auth4" : false, // ip
    "auth5" : false, // geo-location
  }

  constructor(private _http:HttpClient, private _snackBar: MatSnackBar, private router: Router,private _formBuilder: FormBuilder,  private _messagingService:MessagingService) {
    
    this.firstFormGroup = this._formBuilder.group({
      username:[null,Validators.compose([Validators.required])],
      password:[null,Validators.compose([Validators.required])],
    });
   }

  ngOnInit(): void {
    this._messagingService.setToken("");
  }

  auth_1(auth:any) {
    console.log(auth);
    let username = auth.username;
    let user_data = {
      "username" : username,
      "password" : this.encryptUsingTripleDES(auth.password,true)
    }

    let payloadString = JSON.stringify(user_data)
    this._messagingService.sign_in(payloadString).subscribe(
      (response:any) => {
        if (response.status == 200) {
          let msg = response.message.split(",");
          let status_message = msg[0];
          if (status_message == "Success") {
            const chats = msg.slice(1, msg.length + 1);   
            let token = {
                "username" : username,
                "chats" : chats,
                "is_valid_user" : true,
                "is_active_user" : true,
                "open_chat" : chats[0],
            }
            this._messagingService.setToken(token);
            this.router.navigate(['/chatList'])       
          }
        }
        else {
          this.openSnackBar("Invalid username or Password","okay");
        }
      },
      (error) => {
        console.error(error);      
        this.openSnackBar("Invalid username or Password","okay");
      });
  }

 
  openSnackBar(message: string, action: string) {
    this._snackBar.open(message, action, {
      horizontalPosition: "center",
      verticalPosition: "top",
      duration: 2000,
      panelClass: ['custom_snackbar']
    });
  }


  encrypt(text:any,iv:any) {
    const key = 'Thats my Kung Fu'
    const cryptoInfo = CryptoJS.AES.encrypt(JSON.stringify(text), key, {
      mode: CryptoJS.mode.CBC,
      padding: CryptoJS.pad.Pkcs7,
      iv : iv
    }).toString();
    return cryptoInfo
  }

  
  // credits -> https://stackblitz.com/edit/encryption-decryption-triple-des-angular-wo1k7m?file=src%2Fapp%2Fapp.component.ts,src%2Fapp%2Fencryption.service.ts
  key: any = "MTIzNDU2Nzg5MEFCQ0RFRkdISUpLTE1O";
  IV = "MTIzNDU2Nzg=";

  // ENCRYPTION USING CBC TRIPLE DES
  encryptUsingTripleDES(res: any, typeObj: boolean): string {
    const data = typeObj ? JSON.stringify(res) : res;
    const keyHex = CryptoJS.enc.Utf8.parse(this.key);
    const iv = CryptoJS.enc.Utf8.parse(this.IV);
    const mode = CryptoJS.mode.CBC;
    const encrypted = CryptoJS.TripleDES.encrypt(data, keyHex, { iv, mode });
    return encrypted.toString();
  }

  // DECRYPTION USING CBC TRIPLE DES
  decryptUsingTripleDES(encrypted: string): string {
    const keyHex = CryptoJS.enc.Utf8.parse(this.key);
    const iv = CryptoJS.enc.Utf8.parse(this.IV);
    const mode = CryptoJS.mode.CBC;
    const decrypted = CryptoJS.TripleDES.decrypt(encrypted, keyHex, { iv, mode });
    return decrypted.toString(CryptoJS.enc.Utf8);
  }

}