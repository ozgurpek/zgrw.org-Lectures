class Subject {
  private observers: Observer[] = [];
  private a = 0;
  public register(observer: Observer): void {
    this.observers.push(observer);
  }
  public setA(value: number): void {
    this.a = value;
    this.observers.forEach((observer) => observer.update());
  }
  public getA(): number {
    return this.a;
  }
}
class Observer {
  public constructor(
    protected subject: Subject,
    protected b: number,
  ) {
    subject.register(this);
  }
  public update(): void {
    console.log(`I am observer a = ${this.subject.getA()}, b = ${this.b}`);
  }
}
class DivObserver extends Observer {
  public update(): void {
    console.log(`${this.subject.getA()} / ${this.b}`);
  }
}
class ModObserver extends Observer {
  public update(): void {
    console.log(`${this.subject.getA()} % ${this.b}`);
  }
}
const subject = new Subject();
new DivObserver(subject, 3);
new ModObserver(subject, 7);
new DivObserver(subject, 4);
new ModObserver(subject, 5);
subject.setA(12);
