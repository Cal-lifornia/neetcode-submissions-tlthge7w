class Solution {
public:
    bool hasDuplicate(vector<int>& nums) {
        std::map<int, int> seen;
        for (int num: nums) {
            if (seen[num]) return true;
            else seen[num]++;
        }
        return false;
    }
};