import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { timer, Subscription } from 'rxjs';
import { LoginComponent } from './component/login/login/login.component';
import { LoginService } from './services/login/login.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {

  loginStatus: boolean;

  constructor(private loginService: LoginService) {
    this.loginService.isLoggedIn.subscribe(value => this.loginStatus = value);
    this.loginService.ping();
  }

  ngOnInit() {
  }

  logout() {
    this.loginService.logout();
  }

}
