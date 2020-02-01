import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { HttpClientModule } from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { RouterModule, Routes } from '@angular/router';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { DataService } from './services/data/data.service';
import { LoginService } from './services/login/login.service';
import { LoginComponent } from './component/login/login/login.component';
import { NavbarComponent } from './component/partial/navbar/navbar.component';
import { PasswordresetComponent } from './component/login/passwordreset/passwordreset.component';
import { DemoComponent } from './component/demo/demo.component';
import { Demo2Component } from './component/demo2/demo2.component';
import { Demo3Component } from './component/demo3/demo3.component';
import { Demo1Component } from './component/demo1/demo1.component';
import { NotfoundComponent } from './component/notfound/notfound.component';

@NgModule({
  declarations: [
    AppComponent,
    LoginComponent,
    NavbarComponent,
    PasswordresetComponent,
    NotfoundComponent,
    DemoComponent,
    Demo2Component,
    Demo3Component,
    Demo1Component
  ],
  imports: [
    BrowserModule,
    HttpClientModule,
    AppRoutingModule,
    FormsModule,
    AppRoutingModule
  ],
  providers: [LoginService, DataService],
  bootstrap: [AppComponent]
})
export class AppModule { }
