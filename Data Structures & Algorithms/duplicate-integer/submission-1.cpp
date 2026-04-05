class Solution {
public:
    bool hasDuplicate(vector<int>& nums) {
        bool duplicate = false;
        for (int idx=0;idx < nums.size(); idx++) {
            bool should_break = false;
            for (int ydx=idx+1; ydx<nums.size();ydx++) {
                if (nums[idx] == nums[ydx]) {
                    duplicate = true;
                    should_break = true;
                    break;
                }
            }
            if (should_break) {
                break;
            }
        }
        return duplicate;
    }
};