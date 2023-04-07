import { Component, OnInit } from '@angular/core';
import {FormBuilder, FormGroup, Validators} from '@angular/forms';
import * as CryptoJS from 'crypto-js';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import {MatSnackBar} from '@angular/material/snack-bar';

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.css']
})
export class RegisterComponent implements OnInit {

  firstFormGroup: FormGroup;
  imageData : any;
  user : any;

  constructor(private _snackBar: MatSnackBar, private router: Router, private _http:HttpClient, private _formBuilder: FormBuilder) {
    this.firstFormGroup = this._formBuilder.group({
      UserName:[null,Validators.compose([Validators.required])],
      Password:[null,Validators.compose([Validators.required])],
      name:[null,Validators.compose([Validators.required])],
      email:[null,Validators.compose([Validators.required])],
      phone:[null,Validators.compose([Validators.required])],
    });
   }

  ngOnInit(): void {
  }

  new(user:any) {
    this.user = {
      "username" : user.UserName,
      "password" : this.encryptUsingTripleDES(user.Password,true),
      "phone" : this.encryptUsingTripleDES(user.phone,true),
      "name" : this.encryptUsingTripleDES(user.name,true),
      "email" : this.encryptUsingTripleDES(user.email,true),
      "hash" :"ab",
      "chats" :""
    }

    console.log(this.user,"\n",JSON.stringify(this.user));
    this._http.get("http://127.0.0.1:8081/sign_up?"+JSON.stringify(this.user)).subscribe((res:any)=>{
      console.log(res)
      if (res["status"] == 200) {
        this._snackBar.open("✔️ Saved", "close", {
          verticalPosition: "top",
          panelClass: ['color-snackbar']
        });
      }
    });
    this.router.navigate(['/sign_in']);

  }


  handleFileInput(event: any) {

    const reader = new FileReader();
      const [file] = event.target.files;
      reader.readAsDataURL(file);
      reader.onload = () => {
        this.imageData = reader.result;
      };
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
