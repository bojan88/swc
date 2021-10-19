class BaseA {
    createInstance() {
        new BaseA(1);
    }
    constructor(x){
        this.x = x;
    }
}
class BaseB {
    createInstance() {
        new BaseB(2);
    }
    constructor(x){
        this.x = x;
    }
}
class BaseC {
    createInstance() {
        new BaseC(3);
    }
    static staticInstance() {
        new BaseC(4);
    }
    constructor(x){
        this.x = x;
    }
}
class DerivedA extends BaseA {
    createInstance() {
        new DerivedA(5);
    }
    createBaseInstance() {
        new BaseA(6);
    }
    static staticBaseInstance() {
        new BaseA(7);
    }
    constructor(x){
        super(x), this.x = x;
    }
}
class DerivedB extends BaseB {
    createInstance() {
        new DerivedB(7);
    }
    createBaseInstance() {
        new BaseB(8);
    }
    static staticBaseInstance() {
        new BaseB(9);
    }
    constructor(x){
        super(x), this.x = x;
    }
}
class DerivedC extends BaseC {
    createInstance() {
        new DerivedC(9);
    }
    createBaseInstance() {
        new BaseC(10);
    }
    static staticBaseInstance() {
        new BaseC(11);
    }
    constructor(x){
        super(x), this.x = x;
    }
}
new BaseA(1), new BaseB(1), new BaseC(1), new DerivedA(1), new DerivedB(1), new DerivedC(1);