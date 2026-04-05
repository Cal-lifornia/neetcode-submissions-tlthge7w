class Solution {
public:
    bool isValid(string s) {
        std::vector<char> chars= {}; 
        bool result = true;
        int ch= 0;
        bool should_break = false;
        while ( ch<s.size() && !should_break) {
            switch (s[ch])
            {
                case '{':
                    chars.push_back('{');
                    break;
                case '}':
                    if (chars.empty() || chars.back() != '{') {
                        result = false;
                        should_break = true;
                    } else {
                        chars.pop_back();
                    }
                    break;
                case '(':
                    chars.push_back('(');
                    break;
                case ')':
                    if (chars.empty() || chars.back() != '(') {
                        result = false;
                        should_break = true;
                    } else {
                        chars.pop_back();
                    }
                    break;
                case '[':
                    chars.push_back('[');
                    break;
                case ']':
                    if (chars.empty() || chars.back() != '[') {
                        result = false;
                        should_break = true;
                    } else {
                        chars.pop_back();
                    }
                    break;
                default:
                    break;
            }
            ch++;
        }
        if (!chars.empty()) result = false;
        return result;
    }
};
