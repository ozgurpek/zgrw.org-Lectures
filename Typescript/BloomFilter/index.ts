/**
Very simple bloom filter algorithm demonstration.
Hash functions are purposely selected to be simple and colliding
 to demonstrate the potential crashes. See:
 - cars
 - scar
*/


class BloomDatabase {
    private static LENGTH = 100;
    private arr: boolean[] = new Array(BloomDatabase.LENGTH);

    public lookup(str: string): boolean {
        const h1 = this.h1(str);
        const h2 = this.h2(str);
        const h3 = this.h3(str);

        return this.arr[h1] && this.arr[h2] && this.arr[h3];
    }

    public insert(str: string): boolean {
        if (this.lookup(str)) {
            return false;
        }
        const h1 = this.h1(str);
        const h2 = this.h2(str);
        const h3 = this.h3(str);
        this.arr[h1] = true;
        this.arr[h2] = true;
        this.arr[h3] = true;
        return true;
    }

    private sum(str: string): number {
        let res = 0;
        for (let i = 0; i < str.length; ++i) {
            res += str.charCodeAt(i);
        }
        return res;
    }

    private mult(str: string): number {
        let res = 1;
        for (let i = 0; i < str.length; ++i) {
            res *= str.charCodeAt(i);
        }
        return res;
    }

    private subsAbs(str: string): number {
        let res = 1;
        for (let i = 0; i < str.length; ++i) {
            res -= str.charCodeAt(i);
        }
        return Math.abs(res);
    }

    private h1(str: string): number {
        const res = this.sum(str);
        return res % BloomDatabase.LENGTH;
    }

    private h2(str: string): number {
        const res = this.mult(str);
        return res % BloomDatabase.LENGTH;
    }

    private h3(str: string): number {
        const res = this.subsAbs(str);
        return res % BloomDatabase.LENGTH;
    }
}


function tester(db: BloomDatabase, str: string) {
    console.log(`testing: ${str}`);
    let res = db.lookup(str);
    console.log(`lookup ${res ? 'found' : 'not found'} the ${str} in db`);
    res = db.insert(str);
    console.log(`insertion was ${res ? '' : 'not '}successdul`);
}

function main() {
    const db = new BloomDatabase();

    tester(db, 'dog');
    tester(db, 'cat');
    tester(db, 'cars');
    tester(db, 'scar');
}

main()
