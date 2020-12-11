static int local_f(int a) {
    return a * 3;
}

extern int global_f(int a) {
    return a * 6;
}

int main()
{
    int a, b;

    a = 5;
    b = 7;

    b = local_f(a);
    a = global_f(b);

    return a + b; 
}
