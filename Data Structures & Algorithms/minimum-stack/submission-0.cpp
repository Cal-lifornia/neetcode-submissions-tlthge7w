class MinStack {
private: 
    std::vector<int> stack;
public:
    MinStack() {
        
    }
    
    void push(int val) {
       stack.push_back(val); 
    }
    
    void pop() {
        stack.pop_back();
    }
    
    int top() {
        return stack.back();
    }
    
    int getMin() {
        int out = stack[0];
        for (int idx=1; idx< stack.size(); idx++) {
            if (stack[idx] < out) {
                out = stack[idx];
            }
        }
        return out;
    }
};
