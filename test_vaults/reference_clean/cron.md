---
bad_links: 
aliases: [crontab]
tags: [operatingsystems, coding]
---
# Cron

Cron is a time-based job scheduler in Unix-like operating systems. Users can schedule jobs (commands or scripts) to run periodically at fixed times, dates, or intervals. It is typically used for system maintenance tasks, such as rotating log files, backing up data, updating software, and so on.

The name "Cron" comes from the Greek word for time, "chronos". A cron job is the scheduled task itself, which consists of a series of commands executed by the system at a predefined time/date.

Cron uses a configuration file called a crontab, short for "cron table", to manage the scheduling of jobs. Each user can have their own crontab, and though these are files in /var, they are not intended to be edited directly. Instead, users should use the `crontab` command to view or edit their cron jobs.

The syntax of a cron command is as follows:

```
*     *     *   *    *        command to be executed
-     -     -   -    -
|     |     |   |    |
|     |     |   |    +----- day of the week (0 - 6) (Sunday=0)
|     |     |   +------- month (1 - 12)
|     |     +--------- day of the month (1 - 31)
|     +----------- hour (0 - 23)
+------------- min (0 - 59)
```

Each field can contain a single value, a range of values, or multiple values separated by commas. An asterisk (*) in a field signifies "every", so if you put an asterisk in the hour field, it means "every hour".

Here's an example of a cron job that would run a script at 3:30 AM every day:

```
30 3 * * * /path/to/script.sh
```

Cron also supports special strings that can be used as shortcuts for common schedules. For example, `@reboot` runs a job once at startup, `@daily` runs a job once a day, and so on.

Cron is a very powerful tool, but it does have some limitations. For example, it's not well-suited for jobs that need to run at irregular intervals, or jobs that depend on system events or the completion of other jobs. For these types of tasks, other tools like `anacron` or `systemd` timers might be more appropriate.

> For more information, you can refer to the following resources:
> - [Cron Wikipedia Page](https://www.google.com/search?q=Cron+Wikipedia)
> - [Cron on the Linux Documentation Project](https://www.google.com/search?q=Cron+on+the+Linux+Documentation+Project)
> - [Crontab Man Page](https://www.google.com/search?q=Crontab+Man+Page)