# intertest

Disclaimer: this is currently only a proof of concept and should not be used in production.

This testing framework allows you to express dependencies between tests.
Tests can depend on each other, causing a test to be skipped when any of the tests it depends on fails.

This allows for both more expressive test results and for early failures, shortening the time tests take to run.

Consider a test suite that tests a custom vector implementation.
There might be three tests in it:
1. Test that the constructor works
2. Test that items can be inserted
3. Test that items can be removed again

Should there be a bug introduced in the constructor, in a standard testing framework, you might get up to 3 test failures, that are probably ordered arbitrarily in the test report.
This makes it non-obvious that the actual bug was in the constructor, even though the constructor test might have caught the bug.

In intertest, you can describe those tests to depend on each other. Test 2 can depend on test 1, because you need to construct the vector before you add anything to it.
Then test 3 might depend on both 2 and 1, because the vector needs to first be constructed and then items need to be added before it makes sense to run this test.
```
1 ---> 2 --> 3
|            ^
+------------+
```


