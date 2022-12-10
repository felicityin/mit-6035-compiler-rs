use decafc::compile;

fn main() {
    let code = r#"
        int A[100];
        int length;

        int partition(int p, int r) {
            int x, i, j, t, a, z;

            x = A[p];
            i = p - 1;
            j = r + 1;

            for (z = 0; z < length * length; z++) {
                j = j - 1;
                for (a = 0; a < length; a++) {
                    if (A[j] <= x) {
                        break;
                    }
                    j = j - 1;
                }

                for (a = i + 1; a < length; a++) {
                    if (A[a] >= x) {
                        i = a;
                        break;
                    }
                }

                if (i < j) {
                    t = A[i];
                    A[i] = A[j];
                    A[j] = t;
                } else {
                    return j;
                }
            }
            return -1;
        }

        void quicksort(int p, int r) {
            int q;

            if (p < r) {
                q = partition(p, r);
                quicksort(p, q);
                quicksort(q+1, r);
            }
        }

        void main() {
            int temp, i;

            length = 10; // adjust for sort length

            callout("printf", "creating random array of %d elements\n", length);

            callout("srandom", 17);

            for (i = 0; i < length; i++) {
                temp = callout("random");
                A[i] = temp;
            }

            callout("printf", "\nbefore sort:\n");
            for (i = 0; i < length; i++) {
                callout ("printf", "%d\n", A[i]);
            }

            quicksort (0, length - 1);

            callout("printf", "\nafter sort\n");
            for (i = 0; i < length; i++) {
                callout ("printf", "%d\n", A[i]);
            }
        }
    "#;

    let code = String::from(code);
    compile(&code);
}
