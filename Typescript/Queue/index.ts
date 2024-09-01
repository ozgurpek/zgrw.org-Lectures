class Queue {
    private _arr: string[];
    private _len: number;

    public constructor(size: number) {
        this._arr = new Array();
        this._len = size;
    }

    public insert(data: string) {
        if (this._len > this._arr.length) {
            this._arr.push(data);
        } else {
            throw new Error('Queue is full')
        }
    }

    public delete(): string {
        const res = this._arr[0];
        this._arr.splice(0, 1);
        return res;
    }
}

function main() {
    const q = new Queue(5);
    q.insert('hello');
    q.insert('world');
    q.insert('one');
    q.insert('two');
    q.insert('three');
    try {
        q.insert('four');
    } catch(e) {
        console.log('can not put four', e);
    }
    console.log(q.delete());
    console.log(q.delete());
    console.log(q.delete());
    console.log(q.delete());
    console.log(q.delete());
    q.insert('four');
    console.log(q.delete());
}

main()
