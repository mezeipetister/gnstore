import { Component, OnInit } from '@angular/core';
import { LoginService } from 'src/app/services/login/login.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {

  username: string;
  password: string;
  loginStatus: boolean;

  constructor(private loginService: LoginService) {
    this.loginService.isLoggedIn.subscribe(value => this.loginStatus = value);
    this.loginService.ping();
  }

  ngOnInit() {

  }

  login() {
    this.loginService.login(this.username, this.password);
  }

}
