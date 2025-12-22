    auto zigzag(size_t n, size_t count) {
        if(n==1) return count;
        else if(n%2==0) return zigzag(n/2, count+1);
        else return zigzag(n*3+1);
    }
