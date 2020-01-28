import { Injectable } from '@angular/core';
import { Subject, EMPTY } from 'rxjs';
import { DataService, Login } from '../data/data.service';
import { Ok, Err, Result } from 'ts-results';

@Injectable({
  providedIn: 'root'
})

export class LoginService {

  constructor(private dataService: DataService) {
    this.ping();
  }

  // Notification service
  // Publishin login status to subscribers
  isLoggedIn: Subject<boolean> = new Subject<boolean>();

  ping: () => void = function (): void {
    let token = this.getToken();
    // If user logged in, then notify subscibers
    if (token.ok) {
      this.notifyLoginStatus(true);
    } else {
      this.notifyLoginStatus(false);
    }
  }

  notifyLoginStatus(status: boolean) {
    this.isLoggedIn.next(status);
  }

  setToken(token: string): Result<void, Error> {
    try {
      localStorage.setItem('token', token);
      // Notify login subscribers
      this.notifyLoginStatus(true);
      return Ok.EMPTY;
    } catch (e) {
      // Notify login subscribers
      this.notifyLoginStatus(false);
      return new Err(e);
    }
  }

  getToken: () => Result<string, Error> = function (): Result<string, Error> {
    if (localStorage.getItem('token') != null) {
      return new Ok(localStorage.getItem('token'));
    }
    return new Err(new Error("Token not exists"));
  }

  login: (username: string, password: string) =>
    void = function (username: string, password: string) {
      this.dataService.login(username, password).subscribe((data: Login) =>
        this.setToken(data.token).unwrap());
    }

  logout: () => void = function (): void {
    // Removing stored auth token
    localStorage.removeItem('token');
    // Notify subscribers
    this.notifyLoginStatus(false);
  }

}
