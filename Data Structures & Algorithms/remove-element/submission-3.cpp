class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        vector<int>ans;
        int count = 0;
        for (int idx = 0; idx<nums.size(); idx++) {
            if (nums[idx] != val) {
                ans.push_back(nums[idx]);
                count++;
            }
        }
        nums = ans;
        return count;
    }
};