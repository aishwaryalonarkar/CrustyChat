import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { MessagingService } from '../services/messaging.service';

@Component({
  selector: 'app-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.css']
})
export class HeaderComponent implements OnInit {

  constructor(private router: Router, private _messagingService:MessagingService) { }

  ngOnInit(): void {
    let url = window.location.href;
    url = url.replace("http://localhost:4200","");
    if (this._messagingService.getToken() == "") {
      this.router.navigate(["/sign_in"]);
    }
  }

  login() {
    this.router.navigate(['/'])
  }
  register() {
    this.router.navigate(['/sign_up'])
  }
  logout() {
    this.router.navigate(['/sign_in'])
  }

  
}
