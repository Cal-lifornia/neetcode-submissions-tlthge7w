class MinStack {
private: 
    std::vector<int> stack;
    std::vector<int> minStack;
public:
    MinStack() {
        
    }
    
    void push(int val) {
       if (minStack.empty()) {
            minStack.push_back(val);
       } else {
            if (minStack.back() > val) {
                minStack.push_back(val);
            } else {
                minStack.push_back(minStack.back());
            }
       }
       stack.push_back(val); 
    }
    
    void pop() {
        stack.pop_back();
        minStack.pop_back();
    }
    
    int top() {
        return stack.back();
    }
    
    int getMin() {
        return minStack.back();
    }
};
