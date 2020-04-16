# Contributing to Ockam

We'd love for you to contribute to our source code and to make Ockam even better than it is
today! Here are the guidelines we'd like you to follow:

* [Code of Conduct](#coc)
* [Questions and Problems](#question)
* [Issues and Bugs](#issue)
* [Feature Requests](#feature)
* [Improving Documentation](#docs)
* [Issue Submission Guidelines](#submit)
* [Pull Request Submission Guidelines](#submit-pr)
* [Signing the CLA](#cla)

## <a name="coc"></a> Code of Conduct

Help us keep Ockam open and inclusive. Please read and follow our [Code of Conduct][coc].

## <a name="requests"></a> Questions, Bugs, Features

### <a name="question"></a> Got a Question or Problem?

Do not open issues for general support questions as we want to keep GitHub issues for bug reports
and feature requests. You've got much better chances of getting your question answered on dedicated
support platforms, the best being the [Ockam Community on Slack][slack].

To save your and our time, we will systematically close all issues that are requests for general
support and redirect people to the section you are reading right now.

### <a name="issue"></a> Found an Issue or Bug?

If you find a bug in the source code, you can help us by submitting an issue to our
[GitHub Repository][github]. Even better, you can submit a Pull Request with a fix.

**Please see the [Submission Guidelines](#submit) below.**

### <a name="feature"></a> Missing a Feature?

You can request a new feature by submitting an issue to our [GitHub Repository][github-issues].

If you would like to implement a new feature then consider what kind of change it is:

* **Major Changes** that you wish to contribute to the project should be discussed first in an
  [GitHub issue][github-issues] that clearly outlines the changes and benefits of the feature.
* **Small Changes** can directly be crafted and submitted to the [GitHub Repository][github]
  as a Pull Request. See the section about [Pull Request Submission Guidelines](#submit-pr), and
  for detailed information the [core development documentation][developers].

### <a name="docs"></a> Want a Doc Fix?

If you have a suggestion for the documentation, you can open an issue and outline the problem
or improvement you have - however, creating the doc fix yourself is much better!

If you want to help improve the docs, it's a good idea to let others know what you're working on to
minimize duplication of effort. Create a new issue (or comment on a related existing one) to let
others know what you're working on.

If you're making a small change (typo, phrasing) don't worry about filing an issue first.

For large fixes, please build and test the documentation before submitting the PR to be sure you
haven't accidentally introduced any layout or formatting issues on [Ockam.io/learn][learn]. You should also make sure that your
commit message follows the **[Commit Message Guidelines][developers.commits]**.

## <a name="submit"></a> Issue Submission Guidelines
Before you submit your issue search the archive, maybe your question was already answered.

If your issue appears to be a bug, and hasn't been reported, open a new issue. Help us to maximize
the effort we can spend fixing issues and adding new features, by not reporting duplicate issues.

The "[new issue][github-new-issue]" form contains a number of prompts that you should fill out to
make it easier to understand and categorize the issue.

In general, providing the following information will increase the chances of your issue being dealt
with quickly:

* **Expected Behavior** - Is there documentation or an example that represents the discrepancy?
* **Actual Behavior** - Be sure to explain why this is a bug for you.
* **Steps to Reproduce the Problem** - Code snippets and screen shots are always helpful.
* **Environment** - What hardware, OS, and versions are you using?
* **Suggest a Fix** - if you can't fix the bug yourself, perhaps you can point to what might cause the problem (line of code or commit)

Here is a great example of a well defined issue: *add link here when we have a good example*

## <a name="submit-pr"></a> Pull Request Submission Guidelines
Before you submit your pull request consider the following guidelines:

* Search [GitHub](https://github.com/angular/angular.js/pulls) for an open or closed Pull Request
  that relates to your submission. You don't want to duplicate effort.
* Create the [development environment][developers.setup]
* Make your changes in a new git branch:

    ```shell
    git checkout -b username/feature
    ```

* This is the fun part! Build your feature.
* Follow our [Coding Rules][developers.rules].
* If the changes affect public APIs, change or add relevant [documentation][developers.documentation].
* Run the Ockam [unit tests][developers.tests-unit], and ensure that all tests pass.
* Run *Ockam Linting tool* to check that you have followed the automatically enforced coding rules. If you have formatting errors your PR will be rejected by our build tools.
* Ockam requires that all commits are signed by your PGP key. If signing git commits is new to you, we recommend that you use Krypton. Instructions for signing git commits with Krypton can be found [here][sign].
* Commit your changes using a descriptive commit message that follows our [commit message conventions][developers.commits].
* Push your branch to GitHub:

    ```shell

    git push origin username/feature
    ```

* In GitHub, send a pull request to `ockam:develop`. Several automated triggers will fire once you submit a pull request to Ockam:

  * The Ockam [Contributor License Agreement](#cla) tool will check to see if you've signed our CLA. If you have not yet done so, the PR will be blocked. The fix is easy. Simply click the link in the PR 'Conversation' user experience on github.com and follow the signing steps. Be sure to read the CLA during the process.

  * Look at the 'Checks' user experience on github.com. We trigger a process to check that your code follows the guidelines set out in this contributor guide.

  * If you find that our CI has failed, look into the logs on Azure Pipelines to find out
if your changes caused test failures, the commit message was malformed, etc. If you find that the
tests failed or times out for unrelated reasons, you can ping an Ockam team member in the [Ockam Community on Slack][slack] so that the build can be restarted.

* If we suggest changes, then:

  * Make the required updates.
  * Re-run the Ockam test suite to ensure tests are still passing.
  * Commit your changes to your feature branch (e.g. `username/feature`).
  * Squash your commit history and rebase your feature branch to the current version of `ockam:develop`.

  You can also amend the initial commits and force push them to the branch.

  ```shell
  git rebase develop -i
  git push origin username/feature -f
  ```

  This is generally easier to follow, but separate commits are useful if the Pull Request contains
  iterations that might be interesting to see side-by-side.

  * Push the changes to your GitHub repository (this will update your Pull Request).  

That's it! Thank you for your contribution!


## <a name="cla"></a> Signing the Contributor License Agreement (CLA)

When you submit a Pull Request, a bot will ask you to sign our CLA if you haven't done
so before. This is necessary for documentation changes, too. Be sure to read the CLA!

 It's a quick process, we promise!

* For individuals we have a [simple click-through form][individual-cla] that can be completed at any time.
* If you work for a Corporation and your commits are part of a corporate backed project, please [reach out to the Ockam Team via email][corporate-cla] so we can send your legal department our corporate CLA via Docusign.

### References
Thank you to the AngularJS community, this contributor guide was inspired by your format and content.

[coc]: https://github.com/ockam-network/ockam/blob/master/CODE_OF_CONDUCT.md
[learn]: https://ockam.io/learn
[corporate-cla]: mailto:dev@ockam.io
[individual-cla]: https://ockam.io/cla
[github-issues]: https://github.com/ockam-network/ockam/issues
[github-new-issue]: https://github.com/ockam-network/ockam/issues/new
[github]: https://github.com/ockam-network
[sign]:https://krypt.co/docs/start/code-signing.html
[slack]: https://join.slack.com/t/ockam-community/shared_invite/zt-6dlpvjd3-1Ccu_J7kqTx7DSHYb4IOvw
[developers]: DEVELOPERS.md
[developers.commits]: DEVELOPERS.md#commits
[developers.documentation]: DEVELOPERS.md#documentation
[developers.rules]: DEVELOPERS.md#rules
[developers.setup]: DEVELOPERS.md#setup
[developers.tests-unit]: DEVELOPERS.md#unit-tests
