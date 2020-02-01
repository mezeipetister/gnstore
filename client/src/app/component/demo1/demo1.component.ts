import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-demo1',
  templateUrl: './demo1.component.html',
  styleUrls: ['./demo1.component.css']
})

export class Demo1Component implements OnInit {

  user: User = { name: 'Peti', counter: 0 };

  modUser: (user: User) => void = function (user: User): void {
    user.name = 'Modified';
    user.counter++;
  }

  ngOnInit() { }

}

interface User {
  name: string,
  counter: number
}