class Stack {
    private _arr: string[];
    private _len: number;

    public constructor(size: number) {
        this._arr = new Array();
        this._len = size;
    }

    public push(data: string) {
        if (this._len > this._arr.length) {
            this._arr.push(data);
        } else {
            throw new Error('Stack is full')
        }
    }

    public pop(): string {
        const res = this._arr[this._arr.length - 1];
        this._arr.splice(this._arr.length - 1);
        return res;
    }
}

function main() {
    const st = new Stack(5);
    st.push('hello');
    st.push('world');
    st.push('one');
    st.push('two');
    st.push('three');
    try {
        st.push('four');
    } catch(e) {
        console.log('can not put four', e);
    }
    console.log(st.pop());
    console.log(st.pop());
    console.log(st.pop());
    console.log(st.pop());
    console.log(st.pop());
    st.push('four');
    console.log(st.pop());
}

main()
