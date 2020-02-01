import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { Demo2Component } from './component/demo2/demo2.component';
import { Demo3Component } from './component/demo3/demo3.component';
import { Demo1Component } from './component/demo1/demo1.component';
import { NotfoundComponent } from './component/notfound/notfound.component';


const appRoutes: Routes = [
  { path: '', component: Demo1Component },
  { path: 'demo2', component: Demo2Component },
  { path: 'demo3', component: Demo3Component },
  { path: 'notfound', component: NotfoundComponent },
  { path: '**', redirectTo: '/notfound' }
];

@NgModule({
  imports: [
    RouterModule.forRoot(
      appRoutes,
      { enableTracing: true } // <-- debugging purposes only
    )
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }
