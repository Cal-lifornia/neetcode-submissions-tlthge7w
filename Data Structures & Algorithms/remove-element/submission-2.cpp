class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        vector<int>ans;
        for (int idx = 0; idx<nums.size(); idx++) {
            if (nums[idx] != val) {
                ans.push_back(nums[idx]);
            }
        }
        nums = ans;
        return ans.size();
    }
};