import { Injectable } from '@angular/core';
import { Subject, Observable } from 'rxjs';
import { HttpClient, HttpHeaders } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})

export class DataService {
  constructor(private http: HttpClient) { }

  getQuick(): Observable<Msg> {
    return (new Request(this.http)).get('/quick');
  }

  getLong(): Observable<Msg> {
    return (new Request(this.http)).get('/long');
  }

  login(username: string, password: string): Observable<Login> {
    return (new Request(this.http)).post('/login', { username: username, password: password });
  }

  // TODO: Remove
  counter: number = 0;
  // TODO: Remove
  counterChange: Subject<number> = new Subject<number>();
  // TODO: Remove
  increment() {
    this.counter++;
    this.counterChange.next(this.counter);
  }

}

export interface Msg {
  msg: string,
}

export class Login {
  token: string
}

class Options {
  headers: HttpHeaders;
  constructor(token: string) {
    this.headers = new HttpHeaders({
      'Token': token
    })
  }
}

class Request {
  root: string = 'http://localhost:7000';
  options: Options;
  http: HttpClient;
  constructor(http: HttpClient) {
    this.http = http;
    this.options = new Options('eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJ1aWQiOiJhZG1pbiIsInJoaW5vIjp0cnVlfQ.PpcImetmYA0VZd7fiUR2of/sBe62b3bOlp7uNYlNnEo');
  }
  get<T>(url: string): Observable<T> {
    return this.http.get<T>(this.root + url, this.options);
  }
  post<T>(url: string, body: any): Observable<T> {
    return this.http.post<T>(this.root + url, body);
  }
  put<T>(url: string, body: any): Observable<T> {
    return this.http.put<T>(this.root + url, body);
  }
  delete<T>(url: string): Observable<T> {
    return this.http.delete<T>(this.root + url);
  }
}